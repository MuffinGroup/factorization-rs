use std::iter::Enumerate;
use info_types::InfoTypes;

use chrono::{Local, Timelike};

use crate::info_types;

pub fn log(message: &str, info_type: Option<Enumerate<InfoTypes>>) {
    let hours = Local::now().hour();
    let minutes = Local::now().minute();
    let seconds = Local::now().second();

    match info_type {
        Some(i) => {
            println!("<{}:{}:{} | {i: ?}> {}", hours, minutes, seconds, message);
        }
        None => {

        }
    }
}
