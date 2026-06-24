use crate::records::normalizer::Normalizer;

impl Normalizer {
    pub fn normalizer_destructor(&mut self) {
        // `~Normalizer() = default;` — no custom destructor behavior in C++.
    }
}
