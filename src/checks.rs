fn is_idk_depressivo_variant_simples(input: &str) -> bool {
    // Remove espaços e converte para minúsculas para aceitar todas as 2^8 variações
    let cleaned = input.to_ascii_lowercase().replace(" ", "");
    cleaned == "idkdepressivo"
}

fn clean_idk_variants(input: &str) -> Option<(String, &str)> {
    let lower = input.to_ascii_lowercase();
    let base_variants = vec!["idk", "idka", "ideka"];

    for base in base_variants {
        if lower.starts_with(base) {
            let suffix = &input[base.len()..];
            return Some((base.to_string(), suffix));
        }
    }
    None
}

fn check_punctuation_suffix(
    suffix: &str,
    expected: &str,
    max_len: usize,
    special_case: Option<(&str, &str)>,
) -> bool {
    let trimmed_suffix = suffix.trim();

    if let Some((clean_suffix, other_suffix)) = special_case {
        let cleaned_input = trimmed_suffix.to_ascii_lowercase();
        if (cleaned_input.starts_with(clean_suffix) && cleaned_input.contains(other_suffix))
            || (cleaned_input.starts_with(other_suffix) && cleaned_input.contains(clean_suffix))
        {
            return true;
        }
    }

    let len = trimmed_suffix.len();
    if len == 0 || len > max_len {
        return false;
    }

    trimmed_suffix.chars().all(|c| expected.contains(c))
        && trimmed_suffix.ends_with(expected.chars().last().unwrap())
}

fn is_idk_exclamacao_variant(input: &str) -> bool {
    if let Some((_, suffix)) = clean_idk_variants(input) {
        // IDK! (max 4x!) ou IDK!? (max 4 chars total)
        let max_len = if suffix.contains('?') { 4 } else { 5 };
        return check_punctuation_suffix(suffix, "!", max_len, Some(("!?", "?!")));
    }
    false
}

fn is_idk_interrogacao_variant(input: &str) -> bool {
    if let Some((_, suffix)) = clean_idk_variants(input) {
        // IDK? (max 4x?)
        return check_punctuation_suffix(suffix, "?", 4, None);
    }
    false
}

fn is_idk_reticencias_variant(input: &str) -> bool {
    if let Some((_, suffix)) = clean_idk_variants(input) {
        let trimmed_suffix = suffix.trim();
        let len = trimmed_suffix.len();
        // IDK... (3 a 5 pontos)
        return (3..=5).contains(&len) && trimmed_suffix.chars().all(|c| c == '.');
    }
    false
}

fn is_idk_depressivo_exclamacao_variant(input: &str) -> bool {
    let lower_no_space = input.to_ascii_lowercase().replace(" ", "");
    if lower_no_space.starts_with("idkdepressivo") {
        let suffix_start_index =
            input.to_ascii_lowercase().find("depressivo").unwrap_or(0) + "depressivo".len();
        let suffix = &input[suffix_start_index..];
        let trimmed_suffix = suffix.trim();
        let len = trimmed_suffix.len();

        let is_exclamation = (1..=4).contains(&len) && trimmed_suffix.chars().all(|c| c == '!');
        let is_interrobang_variant =
            trimmed_suffix.contains('!') && trimmed_suffix.contains('?') && len <= 4; // Ex: !?, ?!!, etc.

        return is_exclamation || is_interrobang_variant;
    }
    false
}

fn is_idk_depressivo_interrogacao_variant(input: &str) -> bool {
    let lower_no_space = input.to_ascii_lowercase().replace(" ", "");
    if lower_no_space.starts_with("idkdepressivo") {
        let suffix_start_index =
            input.to_ascii_lowercase().find("depressivo").unwrap_or(0) + "depressivo".len();
        let suffix = &input[suffix_start_index..];
        let trimmed_suffix = suffix.trim();
        let len = trimmed_suffix.len();

        return (1..=4).contains(&len) && trimmed_suffix.chars().all(|c| c == '?');
    }
    false
}

fn is_idk_depressivo_reticencias_variant(input: &str) -> bool {
    let lower_no_space = input.to_ascii_lowercase().replace(" ", "");
    if lower_no_space.starts_with("idkdepressivo") {
        let suffix_start_index =
            input.to_ascii_lowercase().find("depressivo").unwrap_or(0) + "depressivo".len();
        let suffix = &input[suffix_start_index..];
        let trimmed_suffix = suffix.trim();
        let len = trimmed_suffix.len();

        return (3..=5).contains(&len) && trimmed_suffix.chars().all(|c| c == '.');
    }
    false
}

pub fn check_idk_simples(input: &str) -> bool {
    input.eq_ignore_ascii_case("idk")
}

pub fn check_idka_simples(input: &str) -> bool {
    input.eq_ignore_ascii_case("idka")
}

pub fn check_ideka_simples(input: &str) -> bool {
    input.eq_ignore_ascii_case("ideka")
}

pub fn check_idk_depressivo_simples(input: &str) -> bool {
    is_idk_depressivo_variant_simples(input)
}

pub fn check_idk_exclamacao(input: &str) -> bool {
    is_idk_exclamacao_variant(input)
}

pub fn check_idk_interrogacao(input: &str) -> bool {
    is_idk_interrogacao_variant(input)
}

pub fn check_idk_reticencias(input: &str) -> bool {
    is_idk_reticencias_variant(input)
}

pub fn check_idk_depressivo_exclamacao(input: &str) -> bool {
    is_idk_depressivo_exclamacao_variant(input)
}

pub fn check_idk_depressivo_interrogacao(input: &str) -> bool {
    is_idk_depressivo_interrogacao_variant(input)
}

pub fn check_idk_depressivo_reticencias(input: &str) -> bool {
    is_idk_depressivo_reticencias_variant(input)
}
