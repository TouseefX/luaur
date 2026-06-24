use crate::records::error_converter::ErrorConverter;
use crate::records::non_strict_function_definition_error::NonStrictFunctionDefinitionError;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_47(&self, e: &NonStrictFunctionDefinitionError) -> String {
        let mut result = String::new();
        if !e.functionName().is_empty() {
            result.push_str("in the function '");
            result.push_str(e.functionName());
            result.push_str("', '");
        }
        result.push_str("the argument '");
        result.push_str(e.argument());
        result.push_str("' is used in a way that will error at runtime");
        result
    }
}
