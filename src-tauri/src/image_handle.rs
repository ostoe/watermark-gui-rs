use std::{
    io::{BufReader, Read},
    path::{Path, PathBuf, self}, collections::HashMap,
};

use crate::{
    banner_unit::{Notification, UserOperation, UserSetting},
    image_processing,
};
use crossbeam_channel::{Receiver, Sender};
use image::{DynamicImage, GenericImageView, GenericImage};
use rusttype::Font;

pub fn control_center_thread(
    operation_st: Receiver<UserOperation>,
    notify_front_st: Sender<Notification>,
) {
    let BRANDS= ["nikon", "canon", "sony", "panasonic", "fujifilm"];
    if let UserOperation::Init(resources_path) = operation_st.recv().unwrap() {
        let (mut font, mut brand_map) = _init(resources_path, &BRANDS);
        let mut is_pause = true;
        let mut image_list = Vec::<String>::new(); // Vec<String>
        let mut image_length = 0usize;
        let mut index = 0usize;
        // user params: --------------------------------begin
        let mut output_path = String::from("");
        let mut qulity: u8 = 85;
        let mut brand = String::from("nikon");
        let mut auto_user_brand = true;
        let mut watermark_ratio = 0.1172f32 * 0.8;
        let mut WATERMARK_SCALE = 0.50;
        let mut logo_ratio = 0.70f32;
        let mut logo_spacing_ratio = 0.35f32;  // if nokon logo should 1
        let mut position_ratio = 0.6267f32;
        let mut split_line_spacing = 30u32; // px doubel = 10 

        // user params: --------------------------------end
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
                UserOperation::Update(user_setting) =>  match user_setting {
                        UserSetting::OutputDir(path) => {
                            output_path = path;
                            println!("outputDir update --> {}", output_path);
                        },
                        UserSetting::Qulity(q) =>  {qulity = q;},
                        UserSetting::AutoUseBrand(is, v) => {
                            if !is {
                                auto_user_brand = false;
                                brand = v;
                            } else {
                                auto_user_brand = true;
                            }
                        },
                        UserSetting::Font(path) => {
                            // update font
                        }

                        _ => { }
                    
                    }

                
                _ => {
                    println!("error option.")
                }
            }
            if !is_pause {
                loop {
                    // process image
                    let image_path = image_list.get(index).unwrap();
                    let start_time = std::time::Instant::now();
                    if let Some(exif_data) = image_processing::read_exif(image_path) {
                        // ImageInfo { width: 1200, height: 800, pixel_format: RGB24, coding_process: DctSequential } 284 
                        // exif_data.get(&rexif::ExifTag::Make). Option::unwrap()` on a `None` value
                        println!("read exif---{:?}", start_time.elapsed());
                        // todo let brand = exif_data.get(&rexif::ExifTag::Make).unwrap();
                        // 
                        let mut tmp_logo_spacing_ratio = logo_spacing_ratio;
                        if auto_user_brand {
                            let camera_make = exif_data.get(&rexif::ExifTag::Make).unwrap();
                            let make_vec: Vec<String> = camera_make.split(" ").map(|x| String::from(x) ).collect();
                            // println!("{:?}", make_vec);
                            brand = make_vec[0].to_lowercase();
                            // *band --> str   &*brand --> &str :::::   String --> &str
                            if brand == "nikon" {
                                tmp_logo_spacing_ratio = 1.0;
                            }
                        }
                        match image_processing::process_single_image(
                            image_path,
                            &output_path,
                            &font,
                            brand_map.get(&*brand).expect("brand error..."),
                            exif_data,
                            qulity,
                            watermark_ratio,
                            WATERMARK_SCALE,
                            tmp_logo_spacing_ratio,
                            position_ratio,
                            logo_ratio,
                            split_line_spacing,
                        ) {
                            Ok(_) => {
                                notify_front_st.send(Notification::Complated).unwrap();
                            }
                            Err(e) => {
                                println!("ImageError: {:?}", e);
                                notify_front_st
                                    .send(Notification::Error(format!(
                                        "{}---图片解码错误{:?}",
                                        image_path, e
                                    )))
                                    .unwrap();
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
                        let opt =
                            notify_front_st.send(Notification::SkipFile(String::from(image_path)));
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
    } else {
        println!("first operation must be initial!!!");
    }
}

fn _init<'a>(path: PathBuf, brands: &'a [&str; 5]) -> (Font<'static>, HashMap<&'a str, DynamicImage>) {
    // read font
    let font_path = path.join("OPPOSans-H.ttf");
    let font_file = std::fs::File::open(font_path).expect("failed to open file");
    let mut font_read = BufReader::new(font_file);
    let mut font: Vec<u8> = vec![];
    let _read_result = font_read.read_to_end(&mut font);
    let font = Font::try_from_vec(font).unwrap();
    //read logo * 3
    let mut map:  HashMap<&str, DynamicImage> = HashMap::new();
    for brand in brands.iter() {
        let brand_with_ex = format!("{}.png", *brand);
        let mut img = image::open(path.join(brand_with_ex)).unwrap();
        for x in 0..img.width() {
            for y in 0..img.height() {
                let mut p = img.get_pixel(x, y);
                if p[3] == 0 {
                    p[0] = 255; p[1]=255; p[2]=255;
                }
                p[3]=0;
                img.put_pixel(x, y, p)
            }
        }
        map.insert(*brand, img);
    }
    // let nikon_banner_img = ;
    // let canon_banner_img = image::open(path.join("canon.png")).unwrap();
    // let sony_banner_img = image::open(path.join("sony.png")).unwrap();
    return (font, map);
}
