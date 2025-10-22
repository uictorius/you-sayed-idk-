use std::collections::HashMap;
use std::fs;
use std::io;

pub enum Language {
    Ar,
    Bn,
    En,
    Es,
    Hi,
    Ja,
    Pa,
    Pt,
    Ru,
    Zh,
}

pub struct Messages {
    pub map: HashMap<String, String>,
}

impl Messages {
    pub fn new(lang: Language) -> io::Result<Self> {
        let file_path = match lang {
            Language::Ar => "i18n/ar.toml",
            Language::Bn => "i18n/bn.toml",
            Language::En => "i18n/en.toml",
            Language::Es => "i18n/es.toml",
            Language::Hi => "i18n/hi.toml",
            Language::Ja => "i18n/ja.toml",
            Language::Pa => "i18n/pa.toml",
            Language::Pt => "i18n/pt.toml",
            Language::Ru => "i18n/ru.toml",
            Language::Zh => "i18n/zh.toml",
        };

        let content = fs::read_to_string(file_path)?;
        let map: HashMap<String, String> =
            toml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(Self { map })
    }

    pub fn get(&self, key: &str) -> &str {
        self.map.get(key).map(|s| s.as_str()).unwrap_or("Key not found")
    }
}
