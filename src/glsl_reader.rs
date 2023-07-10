use std::fs::File;
use std::io::Read;

pub fn read(file_name: &str) -> String {
    let shader_path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/shaders");
    let full_path = format!("{}/{}", shader_path, file_name);
    let mut file = File::open(full_path).expect("Failed to open file");
    let mut file_src = String::new();
    file.read_to_string(&mut file_src)
        .expect("Failed to read file");

    file_src
}