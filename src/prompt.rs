use crate::checks::*;
use crate::print::print;
use crate::translations::Messages;
use std::io;

pub fn prompt(messages: &Messages) {
    loop {
        // Exibe prompt
        print(messages.get("prompt"), "green");

        // Lê input
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            print(messages.get("input_error"), "red");
            continue;
        }
        let input = input.trim();

        // Sai do loop se o usuário digitar "exit" ou equivalente
        if input.eq_ignore_ascii_case(messages.get("exit")) {
            print(messages.get("goodbye"), "cyan");
            break;
        }

        // Avalia entrada
        if check_idk_simples(input) {
            print(messages.get("idk"), "yellow");
        } else if check_idka_simples(input) {
            print(messages.get("idka"), "yellow");
        } else if check_ideka_simples(input) {
            print(messages.get("ideka"), "yellow");
        } else if check_idk_depressivo_simples(input) {
            print(messages.get("idk_depressivo"), "red");
        } else if check_idk_exclamacao(input) {
            print(messages.get("idk_exclamation"), "yellow");
        } else if check_idk_interrogacao(input) {
            print(messages.get("idk_interrogation"), "yellow");
        } else if check_idk_reticencias(input) {
            print(messages.get("idk_ellipsis"), "yellow");
        } else if check_idk_depressivo_exclamacao(input) {
            print(messages.get("idk_depressivo_exclamation"), "red");
        } else if check_idk_depressivo_interrogacao(input) {
            print(messages.get("idk_depressivo_interrogation"), "red");
        } else if check_idk_depressivo_reticencias(input) {
            print(messages.get("idk_depressivo_ellipsis"), "red");
        } else {
            print(messages.get("not_understood"), "red");
        }
    }
}
