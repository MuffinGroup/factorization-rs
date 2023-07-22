#[allow(dead_code)]
pub enum InfoTypes {
    INFO,
    ERROR,
    WARNING
}

impl InfoTypes {
    pub fn literal(&self) -> &str {
        match self {
            InfoTypes::INFO => "Info",
            InfoTypes::ERROR => "Error",
            InfoTypes::WARNING => "Info",
        }
    }
}

#[macro_export]
macro_rules! log {
    ($message: expr, $info_type: path) => {
        let (hours, minutes, seconds) = get_current_time();
        println!("<{}:{}:{}|{}> {}", hours, minutes, seconds, $info_type.literal(), $message);
    };
    ($message: expr) => {
        log!($message, $crate::logger::InfoTypes::INFO)
    };
}