#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::io::{BufReader, Read};

use app::read_exif;
use image::DynamicImage;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, State};
use tauri::{Manager, Window};
use rusttype::{Font, Scale};
// the payload type must implement `Se&rialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}



fn main() {
    let (font, (n_logo, c_logo, s_logo)) = _init();
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);

    tauri::Builder::default()
        .menu(menu)
        .manage((font, (n_logo, c_logo, s_logo)))
        .invoke_handler(tauri::generate_handler![close_splashscreen, greet, send_event])
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
fn greet(name: &str) -> String {
    read_exif::read_exif("ff");
    format!("Hello, {}!", name)
}

#[tauri::command]
fn process_image(brand: &str, dir_path: &str, images_list: &[&str] ,state: State<(Font<'static>, (DynamicImage, DynamicImage, DynamicImage))>) {
    if (dir_path != "") {
        // glob -> images_list
        // 
    } else if (images_list.len() == 0) {
         // send { error message }
    }

    let image_list = ["ff", ""];
    
    for image_path in images_list.iter() {
        let exif_data = read_exif::read_exif(image_path).unwrap(); // todo
        
        read_exif::process_single_image(image_path, brand, &state.0, state.1.0, exif_data)
        // todo use image data mark 非统一
        // send( message ) to front
    }

}

#[tauri::command]
fn send_event(window: Window) {
    //   std::thread::spawn(move || {
    // loop {
    window
        .emit(
            "click",
            Payload {
                message: "rust send: ZZZ!".into(),
            },
        )
        .unwrap();
    // }
    //   });
}



fn _init() -> (Font<'static>, (DynamicImage, DynamicImage, DynamicImage)){
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
