pub fn has_suffix(str: &str, suffix: &str) -> bool {
    str.len() >= suffix.len() && &str[str.len() - suffix.len()..] == suffix
}
