use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeFunctionMissing {
    pub(crate) function_name: String,
}

impl TypeFunctionMissing {
    pub const fn new(function_name: String) -> Self {
        Self { function_name }
    }
}

#[allow(non_snake_case)]
impl TypeFunctionMissing {
    pub fn functionName(&self) -> &str {
        &self.function_name
    }
}
