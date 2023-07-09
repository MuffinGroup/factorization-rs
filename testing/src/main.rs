use std::fs::File;
use std::io::Read;

fn main() {
    let test2 = read_file("C:/Users/Admin/rust/factorization-rs/testing/src/vertex_shader.vert");
    println!("{}", &test2.as_str())
}

fn read_file(file_name: &str) -> String {
    let full_path = format!("{}", file_name);
    let logger_path = format!("Loaded shader: {}", full_path);
    println!("{}", logger_path);
    //log(&logger_path, INFO.types());
    let mut file = File::open(full_path).expect("Failed to open file");
    let mut file_src = String::new();
    file.read_to_string(&mut file_src)
        .expect("Failed to read file");

    file_src
}