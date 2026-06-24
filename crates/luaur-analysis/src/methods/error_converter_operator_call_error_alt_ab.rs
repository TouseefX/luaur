use crate::records::error_converter::ErrorConverter;
use crate::records::missing_properties::MissingProperties;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_31(&self, e: &MissingProperties) -> String {
        let sub_type_str =
            crate::functions::to_string_to_string_alt_c::to_string_type_id(e.subType());
        let super_type_str =
            crate::functions::to_string_to_string_alt_c::to_string_type_id(e.superType());

        let mut s = String::from("Table type '");
        s.push_str(&sub_type_str);
        s.push_str("' not compatible with type '");
        s.push_str(&super_type_str);
        s.push_str("' because the former");

        match e.context() {
            crate::records::missing_properties::Context::Missing => {
                s.push_str(" is missing field");
            }
            crate::records::missing_properties::Context::Extra => {
                s.push_str(" has extra field");
            }
        }

        if e.properties().len() > 1 {
            s.push('s');
        }

        s.push(' ');

        let properties = e.properties();
        for i in 0..properties.len() {
            if i > 0 {
                s.push_str(", ");
            }

            if i > 0 && i == properties.len() - 1 {
                s.push_str("and ");
            }

            s.push('\'');
            s.push_str(&properties[i]);
            s.push('\'');
        }

        s
    }
}
