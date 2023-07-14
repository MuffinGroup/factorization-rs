use std::io::Cursor;
use glium::texture::SrgbTexture2d;

use crate::logger::log;
use crate::info_types::InfoTypes::*;

pub fn load_image(image_bytes: &[u8], image_name: &str, display: &glium::Display) -> SrgbTexture2d {
    let logger_path = format!("Loaded image: {}", image_name);
    log(&logger_path, INFO.literal());

    // Load the image from the byte slice
    let image = image::load(Cursor::new(image_bytes), image::ImageFormat::Png)
        .unwrap()
        .to_rgba8();

    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    // Convert to texture
    let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();
    texture
}

pub fn load_shader(file_bytes: &str, file_name: &str) -> String {
    let logger_path = format!("Loaded shader: {}", file_name);
    log(&logger_path, INFO.literal());
    
    file_bytes.to_string()
}