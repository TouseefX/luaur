use crate::records::builtin_type_functions::BuiltinTypeFunctions;

impl BuiltinTypeFunctions {
    /// In C++, this method is deleted to prevent copying.
    /// In Rust, `BuiltinTypeFunctions` does not implement `Clone` or `Copy`,
    /// so an explicit copy constructor is not provided.
    #[allow(dead_code)]
    pub fn builtin_type_functions_builtin_type_functions(&self) {
        panic!("BuiltinTypeFunctions copy constructor is deleted in C++");
    }
}
