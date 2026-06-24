pub fn get_extension(path: &str) -> alloc::string::String {
    match path.rfind(['.', '\\', '/']) {
        Some(dot_index) => {
            if path.as_bytes()[dot_index] == b'.' {
                alloc::string::String::from(&path[dot_index..])
            } else {
                alloc::string::String::new()
            }
        }
        None => alloc::string::String::new(),
    }
}
