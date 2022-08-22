

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage, ImageError};


fn main1() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn main() {
  watermark_to_one_img();
}


#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}


fn watermark_to_one_img() -> Result<(), ImageError> {

  // 不同的平台路径格式不同 注意区分
  // ImageRgb8
  let src_image =  image::open("G:/workspace/watermark-app/src/assets/20220807-_DSC0279-3.jpg")?;
  let (w, h) = src_image.dimensions();
  // 

  let logo =  image::open("G:/workspace/watermark-app/src/assets/nikon.png")?;

  let (logo_w, logo_h) = logo.dimensions();

  println!("image: wh:{}x{} logo: wh{}x{}, color——type: {:?}", w, h, logo_w, logo_h, src_image.color());

  let mut target_imag: RgbImage = ImageBuffer::new(w, h + logo_h);
  target_imag.copy_from(&src_image, 0, 0);
  
  return Ok(());
}