use crate::commands::execute_command;
use crate::print::print;
use crate::translations::Messages;
use std::io;

pub fn prompt(messages: &Messages) {
    // Prompt fixo do shell
    const SHELL_PROMPT: &str = "idk@shell:~$ ";

    loop {
        // Exibe prompt e espera a entrada na mesma linha
        print(SHELL_PROMPT, "green");

        let mut input = String::new();
        // Lê input
        if io::stdin().read_line(&mut input).is_err() {
            print(messages.get("input_error"), "red");
            continue;
        }
        let input = input.trim();

        // Sai do loop se o usuário digitar o comando de saída
        if input.eq_ignore_ascii_case(messages.get("exit")) {
            print(messages.get("goodbye"), "cyan");
            break;
        }

        // Executa o comando, ignorando entradas vazias
        if !input.is_empty() {
            execute_command(input, messages);
        }
    }
}
