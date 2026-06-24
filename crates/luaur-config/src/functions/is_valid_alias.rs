#[allow(non_snake_case)]
pub fn isValidAlias(alias: &str) -> bool {
    if alias.is_empty() {
        return false;
    }

    let alias_is_not_a_path =
        alias != "." && alias != ".." && !alias.contains('\\') && !alias.contains('/');

    if !alias_is_not_a_path {
        return false;
    }

    for (i, ch) in alias.chars().enumerate() {
        if i == 0 && ch == '@' {
            continue;
        }

        let is_upper = ch >= 'A' && ch <= 'Z';
        let is_lower = ch >= 'a' && ch <= 'z';
        let is_digit = ch >= '0' && ch <= '9';

        if !is_upper && !is_lower && !is_digit && ch != '-' && ch != '_' && ch != '.' {
            return false;
        }
    }

    true
}
