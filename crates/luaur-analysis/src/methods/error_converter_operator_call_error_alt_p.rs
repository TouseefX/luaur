use crate::records::code_too_complex::CodeTooComplex;
use crate::records::error_converter::ErrorConverter;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_17(&self, _error: &CodeTooComplex) -> String {
        String::from(
            "Code is too complex to typecheck! Consider simplifying the code around this area",
        )
    }
}
