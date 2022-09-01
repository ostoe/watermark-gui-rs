use std::io::{BufReader, Read};

use crossbeam_channel::{Receiver, Sender};
use image::DynamicImage;
use rusttype::{Font};
use crate::{banner_unit::{UserOperation, Notification}, read_exif};




pub fn control_center_thread(operation_st: Receiver<UserOperation>, notify_front_st: Sender<Notification>) {

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
                let start_time = std::time::Instant::now();
                if let Some(exif_data) = read_exif::read_exif(image_path) {
                    println!("read exif---{:?}", start_time.elapsed());
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



}



pub fn _init() -> (Font<'static>, (DynamicImage, DynamicImage, DynamicImage)) {
    // read font
    let font_path = "../src/assets/OPPOSans-H.ttf";
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