use crate::functions::to_string_to_string_alt_b::to_string_type_id_to_string_options_mut;
use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::functions::to_string_to_string_alt_d::to_string_type_pack_id;
use crate::records::error_converter::ErrorConverter;
use crate::records::to_string_options::ToStringOptions;
use crate::records::types_are_unrelated::TypesAreUnrelated;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_55(&self, e: &TypesAreUnrelated) -> String {
        let mut opts = ToStringOptions::default();
        let left_str = to_string_type_id_to_string_options_mut(e.left, opts);
        let right_str = to_string_type_id(e.right);
        format!(
            "Cannot cast '{}' into '{}' because the types are unrelated",
            left_str, right_str
        )
    }
}
