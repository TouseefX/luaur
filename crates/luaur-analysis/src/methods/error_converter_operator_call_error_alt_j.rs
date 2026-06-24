use crate::records::error_converter::ErrorConverter;
use crate::records::function_does_not_take_self::FunctionDoesNotTakeSelf;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_24(&self, _: &FunctionDoesNotTakeSelf) -> String {
        String::from(
            "This function does not take self. Did you mean to use a dot instead of a colon?",
        )
    }
}
