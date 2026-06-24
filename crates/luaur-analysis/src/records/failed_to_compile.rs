use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FailedToCompile {
    pub(crate) function_name: String,
    pub(crate) compile_error: String,
}

impl FailedToCompile {
    pub const fn new(function_name: String, compile_error: String) -> Self {
        Self {
            function_name,
            compile_error,
        }
    }
}

#[allow(non_snake_case)]
impl FailedToCompile {
    pub fn functionName(&self) -> &str {
        &self.function_name
    }

    pub fn compileError(&self) -> &str {
        &self.compile_error
    }
}
