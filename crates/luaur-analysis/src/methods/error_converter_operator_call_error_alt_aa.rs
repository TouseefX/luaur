use crate::records::error_converter::ErrorConverter;
use crate::records::illegal_require::IllegalRequire;
use alloc::string::String;

impl ErrorConverter {
    #[allow(non_snake_case)]
    pub fn operator_call_28(&self, e: &IllegalRequire) -> String {
        let mut result = String::from("Cannot require module ");
        result.push_str(e.moduleName());
        result.push_str(": ");
        result.push_str(e.reason());
        result
    }
}
