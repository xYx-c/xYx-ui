
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

impl ToString for Sizes {
    fn to_string(&self) -> String {
        match self {
            Sizes::Small => "small".to_string(),
            Sizes::Normal => "normal".to_string(),
            Sizes::Medium => "medium".to_string(),
            Sizes::Large => "large".to_string(),
        }
    }
}
