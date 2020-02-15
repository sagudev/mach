use std::fmt::Debug;

#[derive(Default)]
pub struct DD {
    pub bootstrap: Option<String>,
    pub build: Option<String>,
    pub run: Option<String>,
    pub env: Option<String>,
    pub format: Option<String>,
    pub clean: Option<String>,
    pub test: Option<String>,
    pub doc: Option<String>,
}

pub enum Lang {
    Rust,
    Cpp,
    C,
    Go,
    Mozilla,
    None,
}

impl From<String> for Lang {
    fn from(x: String) -> Self {
        match x.to_lowercase().as_str() {
            "rust" => Lang::Rust,
            "c" => Lang::C,
            "cpp" => Lang::Cpp,
            _ => Lang::None,
        }
    }
}

impl Default for Lang {
    fn default() -> Self {
        Lang::None
    }
}

impl DD {
    pub fn default(lang: Lang) -> Self {
        match lang {
            Lang::Rust => DD {
                build: Some("cargo build".to_string()),
                run: Some("cargo run".to_string()),
                test: Some("cargo test".to_string()),
                format: Some("cargo fmt".to_string()),
                doc: Some("cargo doc".to_string()),
                clean: Some("cargo clean".to_string()),
                ..Default::default()
            },
            _ => DD {
                ..Default::default()
            },
        }
    }
}
