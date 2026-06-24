use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::functions::to_string_to_string_alt_d::to_string_type_pack_id;
use crate::records::ambiguous_function_call::AmbiguousFunctionCall;
use crate::records::error_converter::ErrorConverter;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call(&self, afc: &AmbiguousFunctionCall) -> String {
        let function_str = to_string_type_id(afc.function);
        let arguments_str = to_string_type_pack_id(afc.arguments);
        format!(
            "Calling function {} with argument pack {} is ambiguous.",
            function_str, arguments_str
        )
    }
}
