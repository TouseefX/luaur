use crate::records::error_converter::ErrorConverter;
use crate::records::not_a_table::NotATable;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_34(&self, e: &NotATable) -> String {
        let ty = crate::functions::to_string_to_string_alt_c::to_string_type_id(e.ty);
        String::from("Expected type table, got '") + &ty + "' instead"
    }
}
