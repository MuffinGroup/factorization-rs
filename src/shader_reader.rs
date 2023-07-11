use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

use crate::logger::log;
use crate::info_types::InfoTypes::*;

pub fn read(file_name: &str) -> String {
    let mut shader_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    shader_path.push("resources");
    shader_path.push("shaders");
    shader_path.push(file_name);

    let logger_path = format!("Loaded shader: {:?}", shader_path);
    log(&logger_path, INFO.types());

    let mut file = File::open(shader_path).expect("Failed to open file");
    let mut file_src = String::new();
    file.read_to_string(&mut file_src)
        .expect("Failed to read file");

    file_src
}
