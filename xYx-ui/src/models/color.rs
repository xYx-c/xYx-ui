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

impl Into<&str> for Colors {
    fn into(self) -> &'static str {
        match self {
            Colors::Primary => " is-primary",
            Colors::Link => " is-link",
            Colors::Info => " is-info",
            Colors::Success => " is-success",
            Colors::Warning => " is-warning",
            Colors::Danger => " is-danger",
            Colors::Black => " is-black",
            Colors::Dark => " is-dark",
            Colors::Light => " is-light",
            Colors::White => " is-white",
        }
    }
}
