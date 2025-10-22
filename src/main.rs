// Internal modules
mod translations;
use translations::Messages;

mod commands;
mod print;
use print::{detect_language, print};

mod prompt;
use prompt::prompt;

fn main() {
    // Detecta idioma e carrega mensagens
    let lang = detect_language();
    let messages = match Messages::new(lang) {
        Ok(msg) => msg,
        Err(e) => {
            print(&format!("Erro ao carregar as mensagens: {}", e), "red");
            return;
        }
    };

    // Inicia loop de prompt
    prompt(&messages);
}
