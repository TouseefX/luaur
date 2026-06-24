use crate::records::error_converter::ErrorConverter;
use crate::records::internal_error::InternalError;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_30(&self, e: &InternalError) -> String {
        String::from(e.message())
    }
}
