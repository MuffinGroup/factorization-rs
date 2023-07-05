use std::fs::File;
use std::io::Read;

use crate::info_types::InfoTypes;
use crate::logger::log;

pub fn read(file_name: &str) -> String {
    let shader_path = "resources/shaders";
    let full_path = format!("{}/{}", shader_path, file_name);
    let logger_path = format!("Loaded shader: {}", full_path);
    log(&logger_path, InfoTypes::INFO.info_type());
    let mut file = File::open(full_path).expect("Failed to open file");
    let mut file_src = String::new();
    file.read_to_string(&mut file_src)
        .expect("Failed to read file");

    file_src
}