use crate::translations::Language;
use colored::*;
use locale_config::Locale;

// Detecta idioma do sistema
pub fn detect_language() -> Language {
    let locale = Locale::user_default();
    let lang_str = locale.to_string().to_lowercase();

    // Verifica os dois primeiros caracteres do locale para determinar o idioma
    match &lang_str[..2] {
        "ar" => Language::Ar,
        "bn" => Language::Bn,
        "en" => Language::En,
        "es" => Language::Es,
        "hi" => Language::Hi,
        "ja" => Language::Ja,
        "pa" => Language::Pa,
        "pt" => Language::Pt,
        "ru" => Language::Ru,
        "zh" => Language::Zh,
        _ => Language::En, // fallback padrão
    }
}
// Imprime texto colorido
pub fn print(text: &str, color: &str) {
    match color {
        "red" => println!("{}", text.red()),
        "green" => println!("{}", text.green()),
        "yellow" => println!("{}", text.yellow()),
        "blue" => println!("{}", text.blue()),
        "magenta" => println!("{}", text.magenta()),
        "cyan" => println!("{}", text.cyan()),
        "white" => println!("{}", text.white()),
        _ => println!("{}", text),
    }
}
