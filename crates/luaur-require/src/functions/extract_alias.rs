extern crate alloc;

use alloc::string::String;

pub(crate) fn extract_alias(path: &str) -> String {
    // To ignore the '@' alias prefix when processing the alias
    const ALIAS_START_POS: usize = 1;

    // If a directory separator was found, the length of the alias is the
    // distance between the start of the alias and the separator. Otherwise,
    // the whole string after the alias symbol is the alias.
    let alias_len = if let Some(separator_pos) = path.find('/') {
        separator_pos - ALIAS_START_POS
    } else {
        path.len() - ALIAS_START_POS
    };

    let start = ALIAS_START_POS;
    let end = ALIAS_START_POS + alias_len;

    String::from(&path[start..end])
}
