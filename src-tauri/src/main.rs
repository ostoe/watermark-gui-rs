#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::banner_unit::{ImagesPathFromFront, Notification, UserOperation};
use app::read_exif;
use crossbeam_channel::unbounded;
use image::DynamicImage;
use rusttype::{Font};
use std::io::{BufReader, Read};
use tauri::{CustomMenuItem, Menu, MenuItem, State, Submenu};
use tauri::{Manager, Window};
// the payload type must implement `Se&rialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
    state_code: u32,
}

fn main() {
    // image opration channel
    let (st, operation_st) = unbounded::<UserOperation>();
    let (st_clone, _rt_clone) = (st.clone(), operation_st.clone());

    let (notify_front_st, notify_front_rt) = unbounded::<Notification>();
    // let (notify_front_st_clone, notify_front_rt_clone) = (notify_front_st.clone(), notify_front_rt.clone());
    // 图像处理线程，等待处理来自前端的消息，处理完成后发送消息通知至消息处理线程，这时消息处理线程再通知前端。
    let image_handle = std::thread::Builder::new()
        .name("ImageHandle".to_string())
        .spawn(move || {
            let (font, (n_logo, c_logo, s_logo)) = _init();
            let mut is_pause = true;
            let mut image_list = Vec::<String>::new(); // Vec<String>
            let mut image_length = 0usize;
            let mut index = 0usize;
            let mut output_path = String::from("");
            loop {
                let opt = operation_st.recv().unwrap();
                match opt {
                    UserOperation::ImagePath(name) => {
                        // 调用处理图像
                        image_list = name
                            .split("\n")
                            .map(|x| String::from(x))
                            .collect::<Vec<String>>();
                        println!("UserOperation{:?} ", image_list);
                        // gen imagelist_  image_index = 0
                        image_length = image_list.len();
                        index = 0;
                        is_pause ^= true
                    }
                    UserOperation::DirPath(path) => {
                        // gen imagelist_  image_index = 0
                        image_length = 10
                    }

                    UserOperation::Pause => {
                        // xor with true --> ref: https://doc.rust-lang.org/reference/types/boolean.html#logical-xor
                        is_pause ^= true
                    }
                    UserOperation::Cancel => {
                        is_pause = true;
                        image_length = 0usize;
                        index = 0usize;
                    }
                    UserOperation::Update(k, v) => match k.as_str() {
                        "output_dir" => {
                            output_path = v;
                            println!("outputDir update --> {}", output_path);
                        }
                        _ => {
                            println!("unsupported update key... #TODO")
                        }
                    }, // _ => {}
                }
                if !is_pause {
                    loop {
                        // process image
                        let image_path = image_list.get(index).unwrap();
                        if let Some(exif_data) = read_exif::read_exif(image_path) {
                            // todo let brand = exif_data.get(&rexif::ExifTag::Make).unwrap();
                            let brand = "nikon";
                            match read_exif::process_single_image(
                                image_path,
                                &output_path,
                                brand,
                                &font,
                                (&c_logo, &n_logo, &s_logo),
                                exif_data,
                            ) {
                                Ok(_) => {
                                    notify_front_st
                                    .send(Notification::Complated).unwrap();
                                }
                                Err(e) => {
                                    println!("ImageError: {:?}", e);
                                    notify_front_st.send(Notification::Error(format!("{}---图片解码错误{:?}",image_path, e))).unwrap();
                                 }
                            }
                            // img = image_list[index]...;

                            if let Ok(opt) =
                                operation_st.recv_timeout(std::time::Duration::from_millis(1))
                            {
                                if let UserOperation::Pause = opt {
                                    is_pause ^= true;
                                    break;
                                }
                            }


                        } else {
                            println!("skip: {}", &image_path);
                            let opt = notify_front_st
                                .send(Notification::SkipFile(String::from(image_path)));
                        }
                        index += 1;
                        if index >= image_length {
                            index = 0;
                            image_length = 0;
                            is_pause = true;
                            break;
                        }
                    }
                }
            }
        });

    // menu------------------------begin
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);
    // menu------------------------end

    tauri::Builder::default()
        .menu(menu)
        .manage(st)
        .invoke_handler(tauri::generate_handler![
            close_splashscreen,
            handle_front_select_files,
            handle_front_update_data,
            greet,
            send_event
        ])
        .setup(move |app| {
            let main_window = app.get_window("main").unwrap();
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let _control_center = std::thread::Builder::new()
                .name("ControlCenter".to_string())
                .spawn(move || {
                    st_clone
                        .send(UserOperation::Update("ABCD".to_string(), "EFG".to_string()))
                        .unwrap();
                    // let id = main_window.listen("front-backend", |event| {
                    //     println!("got window event-name with payload {:?}", event.payload());
                    // });
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    splashscreen_window.close().unwrap();
                    main_window.show().unwrap();
                    loop {
                        let opt_result = notify_front_rt.recv().unwrap();
                        match opt_result {
                            Notification::Single(fname) => {
                                println!("----{:?}", fname);
                                windows_send_msg(&main_window, "front-backend", &fname, 200);
                            }
                            Notification::Complated => {
                                windows_send_msg(&main_window, "front-backend", "", 200);
                            },
                             Notification::Error(e) => {
                                windows_send_msg(&main_window, "front-backend", &e, 500);
                             },
                             Notification::SkipFile(fname) => {
                                windows_send_msg(&main_window, "front-backend", &fname, 300);
                             }
                        }
                    }
                });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

#[tauri::command]
fn greet(name: &str, state: State<crossbeam_channel::Sender<UserOperation>>) -> String {
    state
        .send(UserOperation::ImagePath(String::from(name)))
        .unwrap();
    format!("Hello, {}!", name)
}

#[tauri::command]
fn handle_front_select_files(
    images_obj: ImagesPathFromFront,
    state: State<crossbeam_channel::Sender<UserOperation>>,
) -> String {
    if images_obj.count != 0 {
        state
            .send(UserOperation::ImagePath(images_obj.image_paths.join("\n")))
            .unwrap();
        // for x in images_obj.image_paths.iter() {
        //TODO verify the path
        // TODO convert univers fle format
        // println!("image path: {}", x);

        // }
    }

    format!("handle_front_select_files, {}!", "handling...")
}

#[tauri::command]
fn handle_front_update_data(
    key: String,
    value: String,
    state: State<crossbeam_channel::Sender<UserOperation>>,
) -> String {
    let a = ["output_dir", "brand"];
    if a.contains(&key.as_str()) {
        state
            .send(UserOperation::Update(String::from("output_dir"), value))
            .unwrap();
        return format!("updating user data");
    }
    return format!("error key.");
}

// #[tauri::command]
// fn process_image(brand: &str, dir_path: &str, images_list: &[&str] ,state: State<(Font<'static>, (DynamicImage, DynamicImage, DynamicImage))>) {
//     if (dir_path != "") {
//         // glob -> images_list
//         //
//     } else if (images_list.len() == 0) {
//          // send { error message }
//     }

//     let image_list = ["ff", ""];

//     for image_path in images_list.iter() {
//         let exif_data = read_exif::read_exif(image_path).unwrap(); // todo

//         read_exif::process_single_image(image_path, brand, &state.0, state.1.0, exif_data)
//         // todo use image data mark 非统一
//         // send( message ) to front
//     }

// }

#[tauri::command]
fn send_event(window: Window) {
    //   std::thread::spawn(move || {
    // loop {
    windows_send_msg(&window, "front-backend", "ZZZ", 200);
    // }
    //   });
}

fn _init() -> (Font<'static>, (DynamicImage, DynamicImage, DynamicImage)) {
    // read font
    let font_path = "../src/assets/FiraCode-Medium.ttf";
    let font_file = std::fs::File::open(font_path).expect("failed to open file");
    let mut font_read = BufReader::new(font_file);
    let mut font: Vec<u8> = vec![];
    font_read.read_to_end(&mut font);
    let font = Font::try_from_vec(font).unwrap();
    //read logo * 3
    let nikon_banner_img = image::open("../src/assets/nikon.png").unwrap();
    let canon_banner_img = image::open("../src/assets/canon.png").unwrap();
    let sony_banner_img = image::open("../src/assets/sony.png").unwrap();

    return (font, (nikon_banner_img, canon_banner_img, sony_banner_img));
}

pub fn windows_send_msg(window: &Window, event: &str, msg: &str, code: u32) {
    window
        .emit(
            event,
            Payload {
                message: String::from(msg),
                state_code: code, 
            },
        )
        .unwrap();
}
