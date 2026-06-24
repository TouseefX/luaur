use crate::records::builtin_types::BuiltinTypes;

impl BuiltinTypes {
    /// In C++, this method is deleted to prevent copying.
    /// In Rust, `BuiltinTypes` does not implement `Clone` or `Copy`,
    /// so an explicit assignment operator is not provided.
    #[allow(dead_code)]
    pub(crate) fn operator_assign(&mut self, _other: &BuiltinTypes) {
        unimplemented!("BuiltinTypes copy assignment is deleted in C++")
    }
}
