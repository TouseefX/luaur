use crate::functions::to_string_type_function_error::to_string_type_function_error;
use crate::records::built_in_type_function_error::BuiltInTypeFunctionError;
use crate::records::error_converter::ErrorConverter;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_2(&self, e: &BuiltInTypeFunctionError) -> String {
        to_string_type_function_error(&e.error)
    }
}
