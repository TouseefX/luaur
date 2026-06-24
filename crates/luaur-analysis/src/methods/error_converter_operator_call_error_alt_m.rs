use crate::records::error_converter::ErrorConverter;
use crate::records::unknown_require::UnknownRequire;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_44(&self, e: &UnknownRequire) -> String {
        if e.modulePath().is_empty() {
            String::from("Unknown require: unsupported path")
        } else {
            let mut result = String::from("Unknown require: ");
            result.push_str(e.modulePath());
            result
        }
    }
}
