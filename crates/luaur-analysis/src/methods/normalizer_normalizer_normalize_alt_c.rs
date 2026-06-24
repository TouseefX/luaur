use crate::records::normalizer::Normalizer;

impl Normalizer {
    /// In C++, the default constructor is deleted.
    /// In Rust, we represent this by providing a method that panics if called.
    #[allow(dead_code)]
    pub fn normalizer(&mut self) {
        panic!("Normalizer default constructor is deleted in C++");
    }
}
