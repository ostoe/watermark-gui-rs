#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#[macro_use]
extern crate log;
use app::banner_unit::{ImagesPathFromFront, Notification, UserOperation, UserSettingsJson, UserSetting};
use app::image_handle::control_center_thread;
use app::image_processing;
use app::notify_center::{ notification_thread, windows_send_msg};
use crossbeam_channel::unbounded;
use env_logger::{Builder, Target};
use image::DynamicImage;
use rusttype::Font;
use std::env;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use tauri::{
    api::path::{home_dir, resource_dir},
    Env, PackageInfo,
};
use tauri::{CustomMenuItem, Menu, MenuItem, State, Submenu};
use tauri::{Manager, Window};


fn main() {
    // println!("current_dir {:?}", std::env::current_dir().unwrap());
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);

    builder.init();
    // image opration channel
    let (st, operation_st) = unbounded::<UserOperation>();
    let (st_clone, _rt_clone) = (st.clone(), operation_st.clone());

    let (notify_front_st, notify_front_rt) = unbounded::<Notification>();
    // let (notify_front_st_clone, notify_front_rt_clone) = (notify_front_st.clone(), notify_front_rt.clone());
    // 图像处理线程，等待处理来自前端的消息，处理完成后发送消息通知至消息处理线程，这时消息处理线程再通知前端。
    let _image_handle = std::thread::Builder::new()
        .name("ImageHandle".to_string())
        .spawn(move || control_center_thread(operation_st, notify_front_st));

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
            handle_front_update_key,
            handle_front_update_user_data,
            handle_front_select_dir,
            greet,
            send_event
        ])
        .setup(move |app| {
            let main_window = app.get_window("main").unwrap();
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let resource_path = app_resources_dir(app.package_info());
            println!("resource_path: {}", resource_path.display());
            st_clone.send(UserOperation::Init(resource_path)).unwrap();
             /* start interactive thread */
            let _control_center = std::thread::Builder::new()
                .name("ControlCenter".to_string())
                .spawn(move || notification_thread(main_window, notify_front_rt));

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
fn handle_front_select_dir(image_dir: String, state: State<crossbeam_channel::Sender<UserOperation>>,) ->  String{

    if &image_dir != "" {
        state
            .send(UserOperation::DirPath(image_dir))
            .unwrap();
    }
    format!("handle_front_select_dir, {}!", "handling...")
}

#[tauri::command]
fn handle_front_update_key(
    key: String,
    value: String,
    state: State<crossbeam_channel::Sender<UserOperation>>,
) -> String {
    let a = ["output_dir", "brand"];
    if a.contains(&key.as_str()) {
        state
            .send(UserOperation::Update(UserSetting::OutputDir(value)))
            .unwrap();
        return format!("updating user data");
    }
    return format!("error key.");
}

#[tauri::command]
fn handle_front_update_user_data(
    user_data: UserSettingsJson,
    state: State<crossbeam_channel::Sender<UserOperation>>,
) -> String {
    // let a = ["output_dir", "brand"];
    println!("{:?}", user_data);
    if user_data.output_dir != "" {
        state
        .send(UserOperation::Update(UserSetting::OutputDir(user_data.output_dir))).unwrap();
    }
    if user_data.qulity != 0 {
        state
        .send(UserOperation::Update(UserSetting::Qulity(user_data.qulity)))
        .unwrap();
    }
    if user_data.auto_user_brand {
        state
        .send(UserOperation::Update(UserSetting::AutoUseBrand(true, String::from("_"))))
        .unwrap();
    } else {
        state.send(UserOperation::Update(UserSetting::AutoUseBrand(false, user_data.brand)))
        .unwrap();
    }
    // #TODO 初次启动批量初始化配置
    state.send(UserOperation::Update(UserSetting::FileNamePattern(user_data.filename_pattern))).unwrap();
    
    return format!("已初始化用户数据..");
}


#[tauri::command]
fn send_event(window: Window) {
    //   std::thread::spawn(move || {
    // loop {
    windows_send_msg(&window, "front-backend", "ZZZ", 200);
    // }
    //   });
}


pub fn app_resources_dir(package_info: &PackageInfo) -> PathBuf {
    let res_dir = resource_dir(package_info, &Env::default())
        .unwrap()
        .join("resources");

    // #[cfg(windows)]
    // unsafe {
    //     RESOURCE_DIR = Some(res_dir.clone());
    // }

    res_dir
}
