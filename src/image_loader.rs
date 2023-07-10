use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Cursor;
use glium::texture::SrgbTexture2d;

use crate::logger::log;
use crate::info_types::InfoTypes::*;

pub fn load_image(image_path: &str, display: &glium::Display) -> SrgbTexture2d {
    // access the current directory
    let texture_path = concat!(env!("CARGO_MANIFEST_DIR"));
    let full_path = format!("{}/{}", texture_path, image_path);
    let logger_path = format!("Loaded image: {}", texture_path);

    log(&logger_path, INFO.types());

    // open the image file
    let mut file = File::open(full_path).expect("Failed to open image file");
    let mut image_bytes = Vec::new();
    file.read_to_end(&mut image_bytes).expect("Failed to read image file");

    // load the image
    let image = image::load(Cursor::new(image_bytes), image::ImageFormat::Png)
        .unwrap()
        .to_rgba8();

    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    //Convert to texture
    let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();
    texture
}
