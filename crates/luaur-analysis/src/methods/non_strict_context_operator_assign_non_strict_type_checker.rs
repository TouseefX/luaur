use crate::records::non_strict_context::NonStrictContext;

impl NonStrictContext {
    /// In C++, this method is deleted to prevent copying.
    /// In Rust, `NonStrictContext` does not implement `Clone` or `Copy`,
    /// so an explicit assignment operator is not provided.
    #[allow(dead_code)]
    pub fn operator_assign(&mut self, _other: &NonStrictContext) -> &mut Self {
        panic!("NonStrictContext is not assignable");
    }
}
