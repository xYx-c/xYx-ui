#[derive(Clone, Copy, Debug)]
pub enum Colors {
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
    Black,
    Dark,
    Light,
    White,
}

impl Default for Colors {
    fn default() -> Self {
        Colors::Primary
    }
}

impl ToString for Colors {
    fn to_string(&self) -> String {
        match self {
            Colors::Primary => "primary".to_string(),
            Colors::Link => "link".to_string(),
            Colors::Info => "info".to_string(),
            Colors::Success => "success".to_string(),
            Colors::Warning => "warning".to_string(),
            Colors::Danger => "danger".to_string(),
            Colors::Black => "black".to_string(),
            Colors::Dark => "dark".to_string(),
            Colors::Light => "light".to_string(),
            Colors::White => "white".to_string(),
        }
    }
}

