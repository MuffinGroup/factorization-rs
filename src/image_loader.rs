use std::fs::File;
use std::io::Read;
use std::io::Cursor;
use glium;
use glium::texture::SrgbTexture2d;

pub fn load_image(image_path: &str, display: &glium::Display) -> SrgbTexture2d {
    println!("{}", image_path);
    let mut file = File::open(image_path).expect("Failed to open image file");
    let mut image_bytes = Vec::new();
    file.read_to_end(&mut image_bytes).expect("Failed to read image file");

    let image = image::load(Cursor::new(image_bytes), image::ImageFormat::Png)
        .unwrap()
        .to_rgba8();

    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();
    texture
}
