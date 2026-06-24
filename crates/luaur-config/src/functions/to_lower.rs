pub(crate) fn to_lower(s: &str) -> alloc::string::String {
    let mut result = alloc::string::String::with_capacity(s.len());

    for b in s.as_bytes() {
        let c = *b;
        if b'A' <= c && c <= b'Z' {
            result.push((c + (b'a' - b'A')) as char);
        } else {
            result.push(c as char);
        }
    }

    result
}
