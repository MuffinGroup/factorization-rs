#[allow(dead_code)]
pub enum InfoTypes {
    INFO,
    ERROR,
    WARNING
}

impl InfoTypes {
    pub fn info_type(&self) -> &str {
        match self {
            InfoTypes::INFO => "Info",
            InfoTypes::ERROR => "Error",
            InfoTypes::WARNING => "Info",
        }
    }
}