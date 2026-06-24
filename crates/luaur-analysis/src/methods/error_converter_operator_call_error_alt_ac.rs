use crate::records::duplicate_generic_parameter::DuplicateGenericParameter;
use crate::records::error_converter::ErrorConverter;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_21(&self, e: &DuplicateGenericParameter) -> String {
        let mut result = String::from("Duplicate type parameter '");
        result.push_str(e.parameterName());
        result.push('\'');
        result
    }
}
