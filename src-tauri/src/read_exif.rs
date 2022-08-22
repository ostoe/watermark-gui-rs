extern crate jpeg_decoder as jpeg;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use image::imageops::FilterType;
use image::{self, DynamicImage, GenericImage};
use image::{ImageBuffer, ImageOutputFormat};
use rexif::parse_buffer;

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
    for entry in exif_parsed.entries.iter() {
        println!(
            "[{:?}] {}: {} --{} ",
            entry.kind, entry.tag, entry.value_more_readable, entry.ifd.tag
        );
    }
    println!("read exif ok");
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
    let (w, h) = (src_img.width(), src_img.height());
    if banner.to_lowercase() == "nikon".to_string() {
        let (banner_w, banner_h) = (453u32, 453u32);
        // LogoSize::Nikon,
        let mut banner_img = image::open("../src/assets/nikon.png").unwrap();
        if (w <= banner_w) || (h <= banner_h) {
            banner_img = banner_img.resize(w / 2, h / 2, FilterType::Gaussian);
        }

        let mut newimg_buf = image::ImageBuffer::new(w as u32, banner_img.height() + h);

        match newimg_buf.copy_from(&src_img, 0, 0) {
            Ok(_) => {}
            Err(e) => {
                // println!("{}", e);
                panic!("{}", e)
            }
        }
        match newimg_buf.copy_from(&banner_img, 0, h) {
            Ok(_) => {}
            Err(e) => {
                // println!("{}", e);
                panic!("{}", e)
            }
        }

        // draw
        use image::{Rgba, RgbImage};
        use imageproc::drawing::{draw_text_mut, text_size};
        use rusttype::{Font, Scale};
        use std::env;
        use std::path::Path;
        let font_path = "../src/assets/DejaVuSans.ttf";

        let font_file = File::open(font_path).expect("failed to open file");
        let mut font_read = BufReader::new(font_file);
        let mut font: Vec<u8> = vec![];
        font_read.read_to_end(&mut font);
        // let font = Vec::from(include_bytes!(font_path) as &[u8]);
        let font = Font::try_from_vec(font).unwrap();

        let height = 24.0;
        let scale = Scale {
            x: height * 2.0,
            y: height,
        };

        let text = "Nikon!!!";
        draw_text_mut(&mut newimg_buf, Rgba([255u8, 0u8, 0u8, 180]), banner_w as i32, h as i32, scale, &font, text);
        let (w, h) = text_size(scale, &font, text);
        println!("Text size: {}x{}", w, h);


        let img_path = "./tests/img/jpg/gps/DSCN0010-99.jpg";
        let mut fout = &mut File::create(&Path::new(&format!("{}.jpg", img_path))).unwrap();
        newimg_buf.write_to(&mut fout, ImageOutputFormat::Jpeg(80))
            .unwrap();
        println!("write ok");
    }
}
