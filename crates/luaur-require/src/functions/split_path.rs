pub fn split_path(path: &str) -> (&str, &str) {
    match path.find('/') {
        Some(pos) => (&path[..pos], &path[pos + 1..]),
        None => (path, ""),
    }
}
