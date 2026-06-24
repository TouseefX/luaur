use crate::records::error_converter::ErrorConverter;
use crate::records::syntax_error::SyntaxError;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_39(&self, e: &SyntaxError) -> String {
        String::from(e.message())
    }
}
