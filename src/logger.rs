use chrono::{Local, Timelike};

pub fn log(message: &str, info_type: &str) {
    let hours = Local::now().hour();
    let minutes = Local::now().minute();
    let seconds = Local::now().second();
    println!("<{}:{}:{}|{}> {}", hours, minutes, seconds, info_type, message);
}
