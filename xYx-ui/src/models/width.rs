pub enum Width {
    Desktop,
    Widescreen,
    Fullhd,
    MaxWidescreen,
}

impl Into<&str> for Width {
    fn into(self) -> &'static str {
        match self {
            Width::Desktop => " is-desktop",
            Width::Widescreen => " is-widescreen",
            Width::Fullhd => " is-fullhd",
            Width::MaxWidescreen => " is-max-widescreen",
        }
    }
}
