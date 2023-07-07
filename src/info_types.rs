#[allow(dead_code)]
pub enum InfoTypes {
    INFO,
    ERROR,
    WARNING
}

impl InfoTypes {
    pub fn types(&self) -> &str {
        match self {
            InfoTypes::INFO => "Info",
            InfoTypes::ERROR => "Error",
            InfoTypes::WARNING => "Info",
        }
    }
}
