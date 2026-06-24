use crate::functions::to_string_to_string::to_string_type_pack_id_to_string_options_mut;
use crate::functions::to_string_to_string_alt_d::to_string_type_pack_id;
use crate::records::error_converter::ErrorConverter;
use crate::records::type_pack_mismatch::TypePackMismatch;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_54(&self, e: &TypePackMismatch) -> String {
        let wanted_str = to_string_type_pack_id(e.wanted_tp);
        let given_str = to_string_type_pack_id(e.given_tp);
        let mut ss =
            String::from("Expected this to be '") + &wanted_str + "', but got '" + &given_str + "'";

        if !e.reason.is_empty() {
            ss += "; ";
            ss += &e.reason;
        }

        ss
    }
}
