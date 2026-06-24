use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CheckedFunctionIncorrectArgs {
    pub(crate) functionName: String,
    pub(crate) expected: usize,
    pub(crate) actual: usize,
}

impl CheckedFunctionIncorrectArgs {
    pub const fn new(function_name: String, expected: usize, actual: usize) -> Self {
        Self {
            functionName: function_name,
            expected,
            actual,
        }
    }
}

#[allow(non_snake_case)]
impl CheckedFunctionIncorrectArgs {
    pub fn functionName(&self) -> &str {
        &self.functionName
    }

    pub fn expected(&self) -> usize {
        self.expected
    }

    pub fn actual(&self) -> usize {
        self.actual
    }
}
