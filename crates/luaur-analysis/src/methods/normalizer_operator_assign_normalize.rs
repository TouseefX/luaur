use crate::records::normalizer::Normalizer;

impl Normalizer {
    /// In C++, this method is deleted to prevent move assignment.
    /// In Rust, `Normalizer` does not implement `Clone` or `Copy`,
    /// and move assignment is prevented by not implementing `Copy`.
    #[allow(dead_code)]
    pub fn operator_assign_mut(&mut self) {
        panic!("Normalizer move assignment is deleted in C++");
    }
}
