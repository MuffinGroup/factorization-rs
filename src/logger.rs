use chrono::{Local, Timelike};

pub fn log(message: &str, info_type: Option<&str>) {
    let hours = Local::now().hour();
    let minutes = Local::now().minute();
    let seconds = Local::now().second();

    match info_type {
        Some(i) => {
            println!("<{}:{}:{}|{}> {}", hours, minutes, seconds, i, message);
        }
        None => {
            println!("test");
        }
    }
}
