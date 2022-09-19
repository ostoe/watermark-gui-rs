extern crate jpeg_decoder as jpeg;
use std::collections::HashMap;

use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

// use banner_unit::{BannerType, BannerUnit};
use image::imageops::{FilterType};
use image::{self, DynamicImage, GenericImage, Pixel};
use image::{ImageBuffer, ImageOutputFormat};
use rexif::{parse_buffer, ExifTag};

// use crate::banner_unit;
use image::{Rgba};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};

pub fn read_exif(img_path: &str) -> Option<HashMap::<ExifTag, String>> {
    // let img_path = "./tests/img/jpg/gps/DSCN0010.jpg";
    // let img_path = "./tests/img/jpg/Canon_40D_photoshop_import.jpg";
    // let img_path = "./tests/img/jpg/Canon_40D.jpg";
    let file = File::open(img_path).expect("failed to open file"); // todo
    let mut decoder = jpeg::Decoder::new(BufReader::new(file));
    let _pixels = decoder.read_info().expect("failed to decode image"); // todo
    // let metadata = decoder.info().unwrap();
    // => metadata ImageInfo { width: 100, height: 77, pixel_format: RGB24, coding_process: DctSequential }
    // image-rs exif信息有限；
    // println!("metadata {:?}", metadata);
    if let Some(exif_data) = decoder.exif_data() {
        // println!( "{} \n", exif_data.len(), );
        let exif_parsed = parse_buffer(exif_data).unwrap();
        // let (w, h) = (metadata.width, metadata.height);
    
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


pub fn process_single_image(img_path: &str, output_path: &str,  font: &Font, font_scale: f32, logo_image: &DynamicImage, 
    exif_map: HashMap::<ExifTag, String>, 
    qulity: u8, watermark_ratio: f32, watermark_scale: f32, logo_spacing_ratio: f32, 
    position_ratio: f32,
    logo_ratio: f32, split_line_spacing: u32, index: &str, file_pattern: &[String; 3]
        ) -> image::ImageResult<()> {
    // convert to BannerStruct to draw..
    //
    
    let start_time = std::time::Instant::now();
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
    let src_img =  image::open(img_path)?;
    // println!("read image---{:?}", start_time.elapsed());

    let focal_length  =  focal_length.split(" ").map(|x|String::from(x)).collect::<Vec<String>>().join("");
    let composit_text = format!("{} {} {} {}", focal_length, f_number, exposure_time, iso);

    let _zzz = "ℤ";
    let  banner_h = src_img.width() as f32 * watermark_ratio;
    
    let (w, h) = (src_img.width(), src_img.height());
    let (ori_logo_w, ori_logo_h) = (logo_image.width(), logo_image.height());
    let new_logo_h = banner_h * logo_ratio * logo_spacing_ratio;
    let new_logo_w = new_logo_h * ori_logo_w as f32 / ori_logo_h as f32;
    println!("banner_h: {} new_w: {} x {}",banner_h, new_logo_w, new_logo_h);
    // clear png alpha
    // resize logo image.
    
    let logo_img: DynamicImage = logo_image.resize(new_logo_w as u32, new_logo_h as u32, FilterType::Triangle);
    let mut newimg_buf = image::ImageBuffer::new(w as u32, banner_h as u32 + h);
    newimg_buf.copy_from(&src_img, 0, 0)?;
    // place white background image which color control by user parameter.
    let white_color: Rgba<u8> = Rgba([255u8, 255u8, 255u8, 0u8]);
    for y in h..(banner_h as u32 + h) {
        for x in 0..w {
            newimg_buf.put_pixel(x, y, white_color);
        }
    }

    // place colume split line.
    let col_color: Rgba<u8> = Rgba([168u8, 168u8, 168u8, 0]);
    let line_shift_h = banner_h * (1.0 - watermark_scale) / 2.0;
    for y in h + line_shift_h  as u32..(h + (banner_h - line_shift_h ) as u32) {
        for x in (w as f32 * position_ratio) as u32..(w as f32 * position_ratio + 2.0) as u32 {
            newimg_buf.put_pixel(x, y, col_color);
        }
    }   
    
    // place logo.png
    newimg_buf.copy_from(&logo_img, (w as f32 * position_ratio - new_logo_w) as u32 - split_line_spacing - 2, 
                    h + ((banner_h - new_logo_h) / 2.0) as u32) ?;

    // text 
    let first_text_y =  line_shift_h  as u32 + h;
    let second_text_y =     h + ((banner_h as f32 * 0.55) as u32); // TODO ????
    let second_text_x =     (w as f32 * position_ratio) as u32 + split_line_spacing;
    let texts = vec![
        ( camera_device, banner_h * 0.25 ,((w as f32 * 0.03) as u32, first_text_y), font_scale, &font, Rgba([0u8, 0u8, 0u8, 0])),  // brand
        ( &composit_text, banner_h * 0.20 ,(second_text_x , first_text_y), font_scale, &font, Rgba([0u8, 0u8, 0u8, 0])), // 20mm f/1.8 1/100 iso 100
        ( data_time, banner_h * 0.18,(second_text_x , second_text_y), font_scale, &font, Rgba([168u8, 168u8, 168u8, 0])), // data
    ];
    // text
    for x in texts.iter() {
        generator_draw_text(x.0, x.1, x.2, x.3, x.4, x.5, &mut newimg_buf);
    }
    //-----------------------------------
    // let (mut banner_w, mut banner_h) = (logo_img.width() as f32, logo_img.height() as f32);
    // let mut banner_h = banner_h as f32 * watermark_scale;
    // let mut temp_banner_img = DynamicImage::new_rgba8(1, 1);
    // if h as f32 / 5.5 < banner_h {
    //     let bg_scale = h as f32 / 5.5 / banner_h;
    //     temp_banner_img =  logo_img.resize((banner_w * bg_scale) as u32 , (banner_h  * bg_scale) as u32, FilterType::Triangle);
    //     logo_img = &temp_banner_img;
    //     banner_h = h as f32 / 5.5;
    // }
    // println!("{} x {}", banner_w, banner_h);
    // place src image
    // println!("draw 162---{:?}", start_time.elapsed());
    // draw text do--> generator_draw_text("f 1.8 1/0 ", 40.0, (banner_w/2, banner_h/3),
    //     1.0, &font, Rgba([179, 63u8, 60u8, 0]), &mut newimg_buf);
    // let img_path = "./tests/DSCN0010-99.jpg";
    /* ======================output_file====================== */
    let output_filename = Path::new(&img_path);
    let file_prefix = output_filename.file_name().unwrap();
    let mut file_name_arr = file_prefix.to_str().unwrap().split(".").collect::<Vec<&str>>();
    file_name_arr.pop();// get filename without suffix ".jpg"
    println!("{:?}, {:?}", file_name_arr, file_pattern);

    let mut target_filename = [String::new(), String::new(), String::new(), String::from(".jpg")];
    //  suffix and preffix
    for i in [0, 2]{
        if file_pattern[i] == "" {
            // do nothing
        } else if file_pattern[i].contains("__custom__") {
            target_filename[i] = file_pattern[i].replace("__custom__", "");
        } else if file_pattern[i].contains("__serial_number__") {
            target_filename[i] = file_pattern[i].replace("__serial_number__", "").replace("$x", index);
        }
    }
    if file_pattern[1].contains("__custom__") {
        target_filename[1] = file_pattern[1].replace("__custom__", "");
    } else if file_pattern[1] == "__keep__" {
        target_filename[1] = file_name_arr.join("");
    }
    
    println!("{:?}", target_filename);

    let output_dir = Path::new(output_path);
    // println!("output: {}-{}-{}",output_dir.display(), file_prefix, filename_suffix);
    let output_path = output_dir.join(target_filename.join(""));
    println!("output: {}", output_path.display());
    // println!("mululljf-->{}", output_dir.display());
    let fout = &mut BufWriter::new(File::create(output_path).unwrap());
    // let mut fout = &mut File::create(output_dir).unwrap();
    newimg_buf
        .write_to( fout, ImageOutputFormat::Jpeg(qulity))
        ?;
    println!("write image---{:?}", start_time.elapsed());
    println!("write ok");
    return Ok(());
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
        x: font_size * scale,
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
