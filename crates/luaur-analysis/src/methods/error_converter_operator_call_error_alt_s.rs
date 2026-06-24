use crate::records::error_converter::ErrorConverter;
use crate::records::generic_error::GenericError;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_27(&self, e: &GenericError) -> String {
        String::from(e.message())
    }
}
