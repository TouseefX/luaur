use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::records::error_converter::ErrorConverter;
use crate::records::extern_type::ExternType;
use crate::records::table_type::TableType;
use crate::records::unknown_property::UnknownProperty;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_43(&self, e: &UnknownProperty) -> String {
        let t = unsafe { follow_type_id(e.table) };
        if unsafe { get_type_id::<TableType>(t) }.is_null() {
            if unsafe { get_type_id::<ExternType>(t) }.is_null() {
                "Type '".to_string()
                    + &to_string_type_id(e.table)
                    + "' does not have key '"
                    + &e.key
                    + "'"
            } else {
                "Key '".to_string()
                    + &e.key
                    + "' not found in external type '"
                    + &to_string_type_id(t)
                    + "'"
            }
        } else {
            "Key '".to_string() + &e.key + "' not found in table '" + &to_string_type_id(t) + "'"
        }
    }
}
