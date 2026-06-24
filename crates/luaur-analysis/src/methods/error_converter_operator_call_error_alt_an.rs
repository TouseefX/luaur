use crate::functions::to_string_to_string_alt_d::to_string_type_pack_id;
use crate::records::error_converter::ErrorConverter;
use crate::records::uninhabited_type_pack_function::UninhabitedTypePackFunction;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_61(&self, e: &UninhabitedTypePackFunction) -> String {
        format!(
            "Type pack function instance {} is uninhabited",
            to_string_type_pack_id(e.tp)
        )
    }
}
