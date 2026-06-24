use crate::type_aliases::type_id::TypeId;
use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NonStrictFunctionDefinitionError {
    pub(crate) function_name: String,
    pub(crate) argument: String,
    pub(crate) argument_type: TypeId,
}

impl NonStrictFunctionDefinitionError {
    pub const fn new(function_name: String, argument: String, argument_type: TypeId) -> Self {
        Self {
            function_name,
            argument,
            argument_type,
        }
    }
}

#[allow(non_snake_case)]
impl NonStrictFunctionDefinitionError {
    pub fn functionName(&self) -> &str {
        &self.function_name
    }

    pub fn argument(&self) -> &str {
        &self.argument
    }

    pub fn argumentType(&self) -> TypeId {
        self.argument_type
    }
}
