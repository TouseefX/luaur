use crate::records::error_converter::ErrorConverter;
use crate::records::user_defined_type_function_error::UserDefinedTypeFunctionError;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_62(&self, e: &UserDefinedTypeFunctionError) -> String {
        String::from(e.message())
    }
}
