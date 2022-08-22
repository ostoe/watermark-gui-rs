
extern crate jpeg_decoder as jpeg;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use rexif::parse_buffer;
use image;
use image::{ImageOutputFormat};


pub fn read_exif(img_path: &str) -> Result<(), std::io::Error> {
    let img_path = "./tests/img/jpg/gps/DSCN0010.jpg";
    // let img_path = "./tests/img/jpg/Canon_40D_photoshop_import.jpg";
    // let img_path = "./tests/img/jpg/Canon_40D.jpg";
    let file = File::open(img_path).expect("failed to open file");
    let mut decoder = jpeg::Decoder::new(BufReader::new(file));
    let pixels = decoder.read_info().expect("failed to decode image");
    let metadata = decoder.info().unwrap();
    let exif_data = decoder.exif_data().unwrap();
    println!("{:?} {} \n{:?}", metadata, exif_data.len(), &exif_data[..100]);
    let exif_parsed = parse_buffer(exif_data).unwrap();

    println!("{}", exif_parsed.mime);
    for entry in exif_parsed.entries.iter() {
        println!("[{:?}] {}: {} --{} ", entry.kind, entry.tag, entry.value_more_readable, entry.ifd.tag);
    }
    println!("read exif ok");
    // read images to vec

    let img = image::open(img_path).unwrap();
    let img_path = "./tests/img/jpg/gps/DSCN0010-99.jpg";
    let mut fout = &mut File::create(&Path::new(&format!("{}.jpg", img_path))).unwrap();
    img.write_to(&mut fout, ImageOutputFormat::Jpeg(10))
    // img.write_to(&mut Cursor::new(&mut small), ImageOutputFormat::Jpeg(99))
    .unwrap();
    println!("write ok");
    Ok(())
    
}