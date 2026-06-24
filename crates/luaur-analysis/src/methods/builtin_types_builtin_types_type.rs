use crate::records::builtin_types::BuiltinTypes;

impl BuiltinTypes {
    /// In C++, this method is deleted to prevent copying.
    /// In Rust, `BuiltinTypes` does not implement `Clone` or `Copy`,
    /// so an explicit copy constructor is not provided.
    ///
    /// Note: This item is a stub for a deleted C++ copy constructor.
    /// The duplicate definition error in this crate is resolved by the overload collapse rule:
    /// the actual constructor implementation lives in the `_alt_c` sibling module.
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn builtin_types_builtin_types_copy_deleted(_other: &BuiltinTypes) {
        unimplemented!("BuiltinTypes copy constructor is deleted in C++");
    }
}
