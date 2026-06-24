use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::records::error_converter::ErrorConverter;
use crate::records::missing_union_property::MissingUnionProperty;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_32(&self, e: &MissingUnionProperty) -> String {
        let mut ss = String::from("Key '");
        ss.push_str(e.key());
        ss.push_str("' is missing from ");

        let mut first = true;
        for ty in e.missing() {
            if first {
                first = false;
            } else {
                ss.push_str(", ");
            }

            ss.push_str("'");
            ss.push_str(&to_string_type_id(*ty));
            ss.push_str("'");
        }

        ss.push_str(" in the type '");
        ss.push_str(&to_string_type_id(e.r#type()));
        ss.push_str("'");

        ss
    }
}
