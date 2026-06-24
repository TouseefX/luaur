use crate::records::normalizer::Normalizer;

impl Normalizer {
    /// In C++, this method is deleted to prevent move construction.
    /// In Rust, `Normalizer` does not implement `Copy`, and move construction
    /// is handled by the language's move semantics.
    #[allow(dead_code)]
    pub fn normalizer_normalizer_mut(&mut self) {
        panic!("Normalizer move construction is deleted in C++");
    }
}
