use glob::glob;
use image::{RgbImage, Rgba, ImageOutputFormat};
use imageproc::drawing::{draw_text_mut, text_size};
use rexif::*;
use rusttype::{Font, Scale};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::io::Read;

const TEST_DIR: &str = "../src/assets";
const PATTERN: &str = "*tf";

// cargo test --color=always --package app --test instance test_font_style -- --exact -Z unstable-options --show-output

#[test]
fn test_font_style() {
    let fonts = glob(
        Path::new(TEST_DIR)
            .join(PATTERN)
            .to_str()
            .expect("Path is not valid unicode."),
    )
    .expect("Failed to read glob pattern")
    .filter_map(Result::ok)
    .collect::<Vec<_>>();
    let fonts_numbers = fonts.len();
    let (h, w) = (30 * fonts_numbers as u32, 800);
    let mut newimg_buf = image::ImageBuffer::new(w, h);

    for y in 0..h {
        for x in 0..w {
            newimg_buf.put_pixel(x, y, Rgba([255u8, 255u8, 255u8, 0u8]));
        }
    }
    // font setting
    let height = 24.0;
    let scale = Scale {
        x: height * 1.0,
        y: height,
    };
    let mut i = 0;
    for font_path in fonts {
        let font_file = File::open(&font_path).expect("failed to open file");
        let mut font_read = BufReader::new(font_file);
        let mut font: Vec<u8> = vec![];
        font_read.read_to_end(&mut font);
        // let font = Vec::from(include_bytes!(font_path) as &[u8]);
        let font = Font::try_from_vec(font).unwrap();

        let s1 = font_path.as_path().display().to_string();
        let text = &(s1.to_owned() + ":ℤ7 Nikon F/1.8 1/100 ISO 100");
        draw_text_mut(
            &mut newimg_buf,
            Rgba([0u8, 0u8, 0u8, 0]),
            50,
            30 * i,
            scale,
            &font,
            text,
        );
        let (w, h) = text_size(scale, &font, text);
        println!("Text size: {}x{}", w, h);
        i += 1;
        // cmp_serialized_exif_with_original(&jpeg)?;
    }
    let img_path = "./tests/font-test-output.jpg";
    let mut fout = &mut File::create(&Path::new(&format!("{}.jpg", img_path))).unwrap();
    newimg_buf
    .write_to(&mut fout, ImageOutputFormat::Jpeg(80))
    .unwrap();
    assert!(true);
}
