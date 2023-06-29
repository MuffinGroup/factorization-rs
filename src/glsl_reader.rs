use std::fs::File;
use std::io::Read;

pub fn read(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("Failed to open file");
    let mut file_src = String::new();
    file.read_to_string(&mut file_src)
        .expect("Failed to read file");

    println!("{}", file_src);

    file_src
}