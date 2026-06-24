use alloc::string::String;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountMismatchContext {
    Arg,
    FunctionResult,
    ExprListResult,
    Return,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CountMismatch {
    pub(crate) expected: usize,
    pub(crate) maximum: Option<usize>,
    pub(crate) actual: usize,
    pub(crate) context: CountMismatchContext,
    pub(crate) is_variadic: bool,
    pub(crate) function: String,
}

impl Default for CountMismatch {
    fn default() -> Self {
        Self {
            expected: 0,
            maximum: None,
            actual: 0,
            context: CountMismatchContext::Arg,
            is_variadic: false,
            function: String::new(),
        }
    }
}

impl CountMismatch {
    pub const Arg: CountMismatchContext = CountMismatchContext::Arg;
    pub const FunctionResult: CountMismatchContext = CountMismatchContext::FunctionResult;
    pub const ExprListResult: CountMismatchContext = CountMismatchContext::ExprListResult;
    pub const Return: CountMismatchContext = CountMismatchContext::Return;

    pub fn expected(&self) -> usize {
        self.expected
    }

    pub fn actual(&self) -> usize {
        self.actual
    }

    pub fn context(&self) -> CountMismatchContext {
        self.context
    }

    pub fn is_variadic(&self) -> bool {
        self.is_variadic
    }
}

impl CountMismatchContext {
    pub const Arg: Self = Self::Arg;
    pub const FunctionResult: Self = Self::FunctionResult;
    pub const ExprListResult: Self = Self::ExprListResult;
    pub const Return: Self = Self::Return;
}
