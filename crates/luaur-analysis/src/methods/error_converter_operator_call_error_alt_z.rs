use crate::functions::to_string_to_string_alt_d::to_string_type_pack_id;
use crate::records::error_converter::ErrorConverter;
use crate::records::function_exits_without_returning::FunctionExitsWithoutReturning;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_25(&self, e: &FunctionExitsWithoutReturning) -> String {
        let expected_type_str = to_string_type_pack_id(e.expected_return_type);
        format!(
            "Not all codepaths in this function return '{}'.",
            expected_type_str
        )
    }
}
