
#[derive(Clone, Copy, Debug)]
pub enum Sizes {
    Small,
    Normal,
    Medium,
    Large,
}

impl Default for Sizes {
    fn default() -> Self {
        Sizes::Normal
    }
}

impl Into<&str> for Sizes {
    fn into(self) -> &'static str {
        match self {
            Sizes::Small => " is-small",
            Sizes::Normal => " is-normal",
            Sizes::Medium => " is-medium",
            Sizes::Large => " is-large",
        }
    }
}
