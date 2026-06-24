use crate::records::normalized_type::NormalizedType;

impl NormalizedType {
    /// In C++, this method is deleted to prevent copying.
    /// In Rust, `NormalizedType` does not implement `Clone` or `Copy`,
    /// so an explicit assignment operator is not provided.
    #[allow(dead_code)]
    pub fn operator_assign(&mut self, _other: &NormalizedType) -> &mut NormalizedType {
        panic!("NormalizedType is not assignable");
    }
}
