use crate::records::cannot_check_dynamic_string_format_calls::CannotCheckDynamicStringFormatCalls;
use crate::records::error_converter::ErrorConverter;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_4(&self, _e: &CannotCheckDynamicStringFormatCalls) -> String {
        String::from("We cannot statically check the type of `string.format` when called with a format string that is not statically known.\nIf you'd like to use an unchecked `string.format` call, you can cast the format string to `any` using `:: any`.")
    }
}
