extern crate jpeg_decoder as jpeg;
use std::collections::HashMap;

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

pub fn read_exif(img_path: &str) -> Result<(), std::io::Error> {
    let img_path = "./tests/img/jpg/gps/DSCN0010.jpg";
    // let img_path = "./tests/img/jpg/Canon_40D_photoshop_import.jpg";
    // let img_path = "./tests/img/jpg/Canon_40D.jpg";
    let file = File::open(img_path).expect("failed to open file");
    let mut decoder = jpeg::Decoder::new(BufReader::new(file));
    let pixels = decoder.read_info().expect("failed to decode image");
    let metadata = decoder.info().unwrap();
    let exif_data = decoder.exif_data().unwrap();
    println!(
        "{:?} {} \n{:?}",
        metadata,
        exif_data.len(),
        &exif_data[..100]
    );
    let exif_parsed = parse_buffer(exif_data).unwrap();
    let (w, h) = (metadata.width, metadata.height);

    println!("{}", exif_parsed.mime);
    let mut exif_map = HashMap::<ExifTag, &str>::new();
    for entry in exif_parsed.entries.iter() {
        match exif_map.entry(entry.tag) {
            std::collections::hash_map::Entry::Vacant(vacant) => {
                let value = entry.value_more_readable.trim();
                vacant.insert(value);
            }
            _ => {}
        }
        println!(
            "[{:?}] {}: {} --{} ",
            entry.kind, entry.tag, entry.value_more_readable, entry.ifd.tag
        );
    }
    // println!("{:?}", exif_map);
    println!("read exif ok");

    // convert to BannerStruct to draw..
    //
    let exposure_time = match exif_map.get(&ExifTag::ExposureTime) {
        Some(v) => *v,
        _ => "0",
    };

    let f_number = match exif_map.get(&ExifTag::FNumber) {
        Some(v) => *v,
        _ => "f/0",
    };
    let camera_device = match exif_map.get(&ExifTag::Model) {
        Some(v) => *v,
        _ => "",
    };
    let iso = match exif_map.get(&ExifTag::ISOSpeedRatings) {
        Some(v) => *v,
        _ => "ISO 0",
    };
    let focal_length = match exif_map.get(&ExifTag::FocalLength) {
        Some(v) => *v,
        _ => "0 mm",
    };
    let data_time = match exif_map.get(&ExifTag::DateTime) {
        Some(v) => *v,
        _ => "",
    };
    // read images to vec
    let img = image::open(img_path).unwrap();
    let img_path = "./tests/img/jpg/gps/DSCN0010-99.jpg";
    // let mut fout = &mut File::create(&Path::new(&format!("{}.jpg", img_path))).unwrap();
    // img.write_to(&mut fout, ImageOutputFormat::Jpeg(10))
    // // img.write_to(&mut Cursor::new(&mut small), ImageOutputFormat::Jpeg(99))
    // .unwrap();
    read_logos("nikon", img);

    Ok(())
}

pub enum LogoSize {
    Nikon, // enum如何表示数字呢？参考image库
    Sony,
    Canon,
}

pub fn read_logos(banner: &str, src_img: DynamicImage) {
    let zzz = "ℤ";
    let (w, h) = (src_img.width(), src_img.height());
    let (mut banner_w, mut banner_h) = (0u32, 0u32);
    let mut banner_img: DynamicImage;
    match banner.to_ascii_lowercase().as_str() {
        "nikon" => {
            print!("nikon");
            (banner_w, banner_h) = (453u32, 453u32);
            // LogoSize::Nikon,
            banner_img = image::open("../src/assets/nikon.png").unwrap();
        }
        _ => {
            print!("_nikon");
            (banner_w, banner_h) = (453u32, 453u32);
            // LogoSize::Nikon,
            banner_img = image::open("../src/assets/nikon.png").unwrap();
        }
    }
    let new_banner_h = (h / 5).min(banner_h);
    let new_banner_w = ((new_banner_h as f32 / banner_h as f32) * banner_w as f32) as u32;
    // if (w <= banner_w) || (h <= banner_h) {
    banner_img = banner_img.resize(new_banner_w, new_banner_h, FilterType::Gaussian);
    // }

    let mut newimg_buf = image::ImageBuffer::new(w as u32, banner_img.height() + h);

    // place src image
    match newimg_buf.copy_from(&src_img, 0, 0) {
        Ok(_) => {}
        Err(e) => {
            // println!("{}", e);
            panic!("{}", e)
        }
    }
    // place white image that color control by user parameter.
    let logo_banner_color: Rgba<u8> = Rgba([255u8, 255u8, 255u8, 0u8]);
    for y in h..(banner_img.height() + h) {
        for x in 0..w {
            newimg_buf.put_pixel(x, y, logo_banner_color);
        }
    }
    // place banner
    match newimg_buf.copy_from(&banner_img, 0, h) {
        Ok(_) => {}
        Err(e) => {
            // println!("{}", e);
            panic!("{}", e)
        }
    }

    // draw text

    let font_path = "../src/assets/DejaVuSans.ttf";

    let font_file = File::open(font_path).expect("failed to open file");
    let mut font_read = BufReader::new(font_file);
    let mut font: Vec<u8> = vec![];
    font_read.read_to_end(&mut font);
    // let font = Vec::from(include_bytes!(font_path) as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let height = 24.0;
    let scale = Scale {
        x: height * 1.5,
        y: height,
    };

    let text = "Nikon!!!";
    draw_text_mut(
        &mut newimg_buf,
        Rgba([255u8, 0u8, 0u8, 180]),
        banner_w as i32,
        h as i32,
        scale,
        &font,
        text,
    );
    let (w, h) = text_size(scale, &font, text);
    println!("Text size: {}x{}", w, h);

    generator_draw_text("f 1.8 1/0 ", 40.0, (banner_w/2, banner_h/3),
        1.0, &font, Rgba([179, 63u8, 60u8, 0]), &mut newimg_buf);

    let img_path = "./tests/img/jpg/gps/DSCN0010-99.jpg";
    let mut fout = &mut File::create(&Path::new(&format!("{}.jpg", img_path))).unwrap();
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
