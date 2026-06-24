use crate::records::builtin_type_functions::BuiltinTypeFunctions;

impl BuiltinTypeFunctions {
    /// In C++, this method is deleted to prevent copying.
    /// In Rust, `BuiltinTypeFunctions` does not implement `Clone` or `Copy`,
    /// so an explicit assignment operator is not provided.
    #[allow(dead_code)]
    pub fn builtin_type_functions_operator_assign(&mut self, _other: &BuiltinTypeFunctions) {
        panic!("BuiltinTypeFunctions copy assignment is deleted in C++");
    }
}
