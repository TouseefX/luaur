use crate::records::error_converter::ErrorConverter;
use crate::records::extra_information::ExtraInformation;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_23(&self, e: &ExtraInformation) -> String {
        String::from(e.message())
    }
}
