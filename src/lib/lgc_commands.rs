use std::str::FromStr;

#[derive(clap::ValueEnum, Clone)]
pub enum Language {
    Python,
    Mojo,
    Julia,
    R,
    Go,
    Rust,
}

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "python" => Ok(Language::Python),
            "mojo" => Ok(Language::Mojo),
            "julia" => Ok(Language::Julia),
            "r" => Ok(Language::R),
            "go" => Ok(Language::Go),
            "rust" => Ok(Language::Rust),
            _ => Err(()),
        }
    }
}
