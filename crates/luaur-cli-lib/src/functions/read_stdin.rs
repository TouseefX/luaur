pub fn read_stdin() -> Option<alloc::string::String> {
    use std::io::{self, Read};

    let mut buffer = String::new();
    let mut stdin = io::stdin();

    match stdin.read_to_string(&mut buffer) {
        Ok(_) => Some(buffer),
        Err(_) => None,
    }
}
