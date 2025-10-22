// commands.rs

use crate::print::print;
use crate::translations::Messages;
use std::process::Command;

pub fn check_idk_simples(input: &str) -> bool {
    input.eq_ignore_ascii_case("idk")
}

// Função principal para analisar o input e executar o comando.
pub fn execute_command(input: &str, messages: &Messages) {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return;
    }

    let command = parts[0];
    let args = &parts[1..];

    // Lógica para Comandos Internos
    match command.to_ascii_lowercase().as_str() {
        "echo" => {
            // Comando 'echo': imprime o resto dos argumentos.
            print(&args.join(" "), "white");
        }
        "ls" => match Command::new("ls").args(args).output() {
            // Comando 'ls': executa o comando 'ls' do sistema.
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                if output.status.success() {
                    print(&stdout, "white");
                } else {
                    print(&stderr, "red");
                }
            }
            Err(_) => print("Erro ao executar 'ls'.", "red"),
        },
        "idk_mode" => {
            // Exemplo de comando interno que usa a lógica 'idk'.
            if check_idk_simples(args.join(" ").trim()) {
                print(messages.get("idk"), "yellow");
            } else {
                print(
                    "idk_mode precisa de um argumento 'idk', 'idka', ou 'ideka'.",
                    "red",
                );
            }
        }
        // Qualquer outra coisa é tratada como um comando externo.
        _ => {
            // Lógica para Comandos Externos
            match Command::new(command).args(args).output() {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if output.status.success() {
                        print(&stdout, "white");
                    } else {
                        // Imprime o erro padrão do sistema
                        print(&stderr, "red");
                        // Se não houver erro de sistema, imprime "Comando não encontrado".
                        if stderr.is_empty() {
                            print(&format!("Comando não encontrado: {}", command), "red");
                        }
                    }
                }
                Err(e) => {
                    // Trata o erro 'Comando não encontrado' (ErrorKind::NotFound)
                    if e.kind() == std::io::ErrorKind::NotFound {
                        // Evita imprimir erro para o comando 'exit', já tratado no 'prompt.rs'
                        if !command.eq_ignore_ascii_case(messages.get("exit")) {
                            print(
                                &format!("{}: {}", messages.get("not_understood"), command),
                                "red",
                            );
                        }
                    } else {
                        // Outros erros de IO
                        print(
                            &format!("Erro ao executar o comando '{}': {}", command, e),
                            "red",
                        );
                    }
                }
            }
        }
    }
}
