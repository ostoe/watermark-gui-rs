extern crate jpeg_decoder as jpeg;
use std::collections::HashMap;

use std::fmt::format;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use banner_unit::{BannerType, BannerUnit};
use image::imageops::{invert, FilterType};
use image::{self, DynamicImage, GenericImage, Pixel};
use image::{ImageBuffer, ImageOutputFormat};
use rexif::{parse_buffer, ExifTag};

use crate::banner_unit;
use image::{RgbImage, Rgba};
use imageproc::drawing::{draw_text_mut, text_size, Canvas};
use rusttype::{Font, Scale};

pub fn read_exif(img_path: &str) -> Option<HashMap::<ExifTag, String>> {
    // let img_path = "./tests/img/jpg/gps/DSCN0010.jpg";
    // let img_path = "./tests/img/jpg/Canon_40D_photoshop_import.jpg";
    // let img_path = "./tests/img/jpg/Canon_40D.jpg";
    let file = File::open(img_path).expect("failed to open file"); // todo
    let mut decoder = jpeg::Decoder::new(BufReader::new(file));
    let _pixels = decoder.read_info().expect("failed to decode image"); // todo
    let metadata = decoder.info().unwrap();
    if let Some(exif_data) = decoder.exif_data() {
        println!(
            "{:?} {} \n",
            metadata,
            exif_data.len(),
            // &exif_data[..100]
        );
        let exif_parsed = parse_buffer(exif_data).unwrap();
        let (w, h) = (metadata.width, metadata.height);
    
        println!("{}", exif_parsed.mime);
        let mut exif_map = HashMap::<ExifTag, String>::new();
        for entry in exif_parsed.entries.into_iter() {
            let tag = entry.tag;
            match exif_map.entry(tag) {
                std::collections::hash_map::Entry::Vacant(vacant) => {
                    let value = entry.value_more_readable.trim();
                    vacant.insert(String::from(value));
                }
                _ => {}
            }
            // println!(
            //     "[{:?}] {}: {} --{} ",
            //     entry.kind, entry.tag, entry.value_more_readable, entry.ifd.tag
            // );
        }
        println!("{:?}", exif_map);
        println!("read exif ok");
        return Some(exif_map);
    } else {
        return None
    }
    
}


pub fn process_single_image(img_path: &str, output_path: &str, brand: &str, font: &Font, brand_image: (&DynamicImage, &DynamicImage, &DynamicImage), 
    exif_map: HashMap::<ExifTag, String>) {
    // convert to BannerStruct to draw..
    //
    let exposure_time = match exif_map.get(&ExifTag::ExposureTime) {
        Some(v) => v,
        _ => "0",
    };

    let f_number = match exif_map.get(&ExifTag::FNumber) {
        Some(v) => v,
        _ => "f/0",
    };
    let camera_device = match exif_map.get(&ExifTag::Model) {
        Some(v) => v,
        _ => "",
    };
    let iso = match exif_map.get(&ExifTag::ISOSpeedRatings) {
        Some(v) => v,
        _ => "ISO 0",
    };
    let focal_length = match exif_map.get(&ExifTag::FocalLength) {
        Some(v) => v,
        _ => "0mm",
    };
    let data_time = match exif_map.get(&ExifTag::DateTime) {
        Some(v) => v,
        _ => "",
    };
    // read images to vec
    let src_img = image::open(img_path).unwrap();

    let focal_length  =  focal_length.split(" ").map(|x|String::from(x)).collect::<Vec<String>>().join("");
    let composit_text = format!("{} {} {} {}", focal_length, f_number, exposure_time, iso);

    let _zzz = "ℤ";
    let (w, h) = (src_img.width(), src_img.height());
    
    let mut banner_img: &DynamicImage = brand_image.1;
    let (mut banner_w, mut banner_h) = (banner_img.width(), banner_img.height());
    let watermark_scale = 1.7;
    let mut background_heigth = banner_h as f32 * watermark_scale;
    let new_bg_h = (h as f32 / 5.0).min(background_heigth);
    // let new_bg_w = (new_bg_h as f32 / banner_h as f32) * banner_w as f32;
    let mut tmp_banner_img = DynamicImage::new_rgba8(1, 1);

    if (new_bg_h <= background_heigth as f32) {
        tmp_banner_img = banner_img.resize((banner_w as f32 * (new_bg_h / background_heigth)) as u32, (banner_h as f32 * (new_bg_h / background_heigth)) as u32, FilterType::Gaussian);
        background_heigth = new_bg_h;
        banner_img = &tmp_banner_img;
        //  ( banner_w,  banner_h) = (banner_img.width(), banner_img.height());
    }
    println!("{} x {}", banner_w, banner_h);
    
    let mut newimg_buf = image::ImageBuffer::new(w as u32, background_heigth as u32 + h);

    // place src image
    match newimg_buf.copy_from(&src_img, 0, 0) {
        Ok(_) => {}
        Err(e) => {
            // println!("{}", e);
            panic!("{}", e)
        }
    }
    // place white image that color control by user parameter.
    let white_color: Rgba<u8> = Rgba([255u8, 255u8, 255u8, 0u8]);
    for y in h..(background_heigth as u32 + h) {
        for x in 0..w {
            newimg_buf.put_pixel(x, y, white_color);
        }
    }
    // place banner
    println!("new image:size {} x {}", newimg_buf.width(), newimg_buf.height() );
    println!("{} {} {}x{}",background_heigth, h + (background_heigth/watermark_scale/2.0 ) as u32 ,banner_img.width(), banner_img.height());
    match newimg_buf.copy_from(banner_img, w/2, h + (background_heigth/watermark_scale/2.0 ) as u32) {
        Ok(_) => {}
        Err(e) => {
            // println!("{}", e);
            panic!("{}", e)
        }
    }

    // draw all text.
    let first_text_y =     h + ((background_heigth as f32 * 0.25) as u32);
    let second_text_y =     h + ((background_heigth as f32 * 0.55) as u32);
    let second_text_x =     (w as f32 * 0.63) as u32;
    let texts = vec![
        ( camera_device, background_heigth * 0.25 ,((w as f32 * 0.03) as u32, first_text_y), 1.0, &font, Rgba([0u8, 0u8, 0u8, 0])),  // brand
        ( &composit_text, background_heigth * 0.20 ,(second_text_x , first_text_y), 1.0, &font, Rgba([0u8, 0u8, 0u8, 0])), // 20mm f/1.8 1/100 iso 100
        ( data_time, background_heigth * 0.20,(second_text_x , second_text_y), 1.0, &font, Rgba([168u8, 168u8, 168u8, 0])), // data
    ];

    for x in texts.iter() {
        generator_draw_text(x.0, x.1, x.2, x.3, x.4, x.5, &mut newimg_buf);
    }

    // place colume split line.
    let col_color: Rgba<u8> = Rgba([168u8, 168u8, 168u8, 0]);
    for y in h + (background_heigth/watermark_scale/2.0) as u32..(h + (background_heigth - background_heigth/watermark_scale/2.0 ) as u32) {
        for x in (w as f32 * 0.60) as u32..(w as f32 * 0.60 + 2.0) as u32 {
            newimg_buf.put_pixel(x, y, col_color);
        }
    }
    // draw text do--> generator_draw_text("f 1.8 1/0 ", 40.0, (banner_w/2, banner_h/3),
    //     1.0, &font, Rgba([179, 63u8, 60u8, 0]), &mut newimg_buf);
    // let img_path = "./tests/DSCN0010-99.jpg";
    let output_filename = Path::new(&img_path);
    let output_dir = Path::new(output_path);
    let file_prefix = output_filename.file_name().unwrap();
    let mut file_name_arr = file_prefix.to_str().unwrap().split(".").collect::<Vec<&str>>();
    file_name_arr.pop();
    let filename_suffix = format!("{}.{}", "-w", "jpg");
    file_name_arr.push(&filename_suffix);
    let file_prefix = file_name_arr.join(".");
    println!("output: {}-{}-{}",output_dir.display(), file_prefix, filename_suffix);
    let output_dir = output_dir.join(file_prefix);
    println!("mululljf-->{}", output_dir.display());
    let mut fout = &mut File::create(output_dir).unwrap();
    newimg_buf
        .write_to(&mut fout, ImageOutputFormat::Jpeg(80))
        .unwrap();
    println!("write ok");
}

fn generator_draw_text(
    text: &str,
    font_size: f32,
    position: (u32, u32),
    scale: f32,
    font: &Font,
    color: Rgba<u8>,
    newimg_buf: &mut ImageBuffer<Rgba<u8>, Vec<<Rgba<u8> as Pixel>::Subpixel>>,
)
{
    // let exposure_time = BannerUnit { banner_type: BannerType::Text(String::from(text)),
    //     size, position};
    // let line_width = 24.0;
    let scale = Scale {
        x: font_size,
        y: font_size * scale,
    };
    draw_text_mut(
        newimg_buf,
        color,
        position.0 as i32,
        position.1 as i32,
        scale,
        &font,
        text,
    );
    let (w, h) = text_size(scale, &font, text);
    println!("Text size: {}x{}", w, h);
}
