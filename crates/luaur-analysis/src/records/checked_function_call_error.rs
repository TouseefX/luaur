use crate::type_aliases::type_id::TypeId;
use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CheckedFunctionCallError {
    pub(crate) expected: TypeId,
    pub(crate) passed: TypeId,
    pub(crate) checkedFunctionName: String,
    pub(crate) argumentIndex: usize,
}

#[allow(non_snake_case)]
impl CheckedFunctionCallError {
    pub const fn new(
        expected: TypeId,
        passed: TypeId,
        checked_function_name: String,
        argument_index: usize,
    ) -> Self {
        Self {
            expected,
            passed,
            checkedFunctionName: checked_function_name,
            argumentIndex: argument_index,
        }
    }

    pub fn expected(&self) -> TypeId {
        self.expected
    }

    pub fn passed(&self) -> TypeId {
        self.passed
    }

    pub fn checkedFunctionName(&self) -> &str {
        &self.checkedFunctionName
    }

    pub fn argumentIndex(&self) -> usize {
        self.argumentIndex
    }
}
