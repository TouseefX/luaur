use crate::records::error_converter::ErrorConverter;
use crate::records::occurs_check_failed::OccursCheckFailed;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_35(&self, _: &OccursCheckFailed) -> String {
        String::from("Type contains a self-recursive construct that cannot be resolved")
    }
}
