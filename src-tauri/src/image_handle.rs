use std::{
    io::{BufReader, Read},
    path::{Path, PathBuf, self}, collections::HashMap,
};

use crate::{
    banner_unit::{Notification, UserOperation, UserSetting},
    image_processing,
};
use crossbeam_channel::{Receiver, Sender};
use image::DynamicImage;
use rusttype::Font;

pub fn control_center_thread(
    operation_st: Receiver<UserOperation>,
    notify_front_st: Sender<Notification>,
) {
    if let UserOperation::Init(resources_path) = operation_st.recv().unwrap() {
        let (mut font, mut brand_map) = _init(resources_path);
        let mut is_pause = true;
        let mut image_list = Vec::<String>::new(); // Vec<String>
        let mut image_length = 0usize;
        let mut index = 0usize;
        let mut output_path = String::from("");
        let mut qulity: u8 = 85;
        let mut brand = String::from("nikon");
        let mut auto_user_brand = true;
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
                        println!("read exif---{:?}", start_time.elapsed());
                        // todo let brand = exif_data.get(&rexif::ExifTag::Make).unwrap();
                        // 
                        if auto_user_brand {
                            let camera_make = exif_data.get(&rexif::ExifTag::Make).unwrap();
                            let make_vec: Vec<String> = camera_make.split(" ").map(|x| String::from(x) ).collect();
                            // println!("{:?}", make_vec);
                            brand = make_vec[0].to_lowercase();
                            // *band --> str   &*brand --> &str :::::   String --> &str
                        }
                        match image_processing::process_single_image(
                            image_path,
                            &output_path,
                            &font,
                            brand_map.get(&*brand).expect("brand error..."),
                            exif_data,
                            qulity,
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

fn _init(path: PathBuf) -> (Font<'static>, HashMap<&'static str, DynamicImage>) {
    // read font
    let font_path = path.join("OPPOSans-H.ttf");
    let font_file = std::fs::File::open(font_path).expect("failed to open file");
    let mut font_read = BufReader::new(font_file);
    let mut font: Vec<u8> = vec![];
    let _read_result = font_read.read_to_end(&mut font);
    let font = Font::try_from_vec(font).unwrap();
    //read logo * 3
    let mut map:  HashMap<&str, DynamicImage> = HashMap::new();
    map.insert("nikon", image::open(path.join("nikon.png")).unwrap());
    map.insert("canon", image::open(path.join("canon.png")).unwrap());
    map.insert("sony", image::open(path.join("sony.png")).unwrap());
    // let nikon_banner_img = ;
    // let canon_banner_img = image::open(path.join("canon.png")).unwrap();
    // let sony_banner_img = image::open(path.join("sony.png")).unwrap();

    return (font, map);
}
