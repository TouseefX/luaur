use crate::records::normalizer::Normalizer;

impl Normalizer {
    /// In C++, this method is deleted to prevent copying.
    /// In Rust, `Normalizer` does not implement `Clone` or `Copy`,
    /// so an explicit assignment operator is not provided.
    #[allow(dead_code)]
    pub fn operator_assign(&mut self, _other: &mut Normalizer) -> &mut Normalizer {
        panic!("Normalizer is not assignable");
    }
}
