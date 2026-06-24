use crate::functions::to_human_readable_index::to_human_readable_index;
use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::records::checked_function_call_error::CheckedFunctionCallError;
use crate::records::error_converter::ErrorConverter;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_5(&self, e: &CheckedFunctionCallError) -> String {
        let mut result = String::from("the function '");
        result.push_str(e.checkedFunctionName());
        result.push_str("' expects to get a ");
        result.push_str(&to_string_type_id(e.expected()));
        result.push_str(" as its ");
        result.push_str(&to_human_readable_index(e.argumentIndex()));
        result.push_str(" argument, but is being given a ");
        result.push_str(&to_string_type_id(e.passed()));
        result
    }
}
