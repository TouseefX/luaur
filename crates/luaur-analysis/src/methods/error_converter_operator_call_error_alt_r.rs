use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::records::error_converter::ErrorConverter;
use crate::records::extern_type::ExternType;
use crate::records::unknown_prop_but_found_like_prop::UnknownPropButFoundLikeProp;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_42(&self, e: &UnknownPropButFoundLikeProp) -> String {
        let mut candidates_suggestion = String::from("Did you mean ");
        if e.candidates().len() != 1 {
            candidates_suggestion.push_str("one of ");
        }

        let mut first = true;
        for name in e.candidates() {
            if first {
                first = false;
            } else {
                candidates_suggestion.push_str(", ");
            }
            candidates_suggestion.push('\'');
            candidates_suggestion.push_str(name);
            candidates_suggestion.push('\'');
        }

        let mut s = String::from("Key '");
        s.push_str(e.key());
        s.push_str("' not found in ");

        let t = unsafe { follow_type_id(e.table()) };
        if unsafe { get_type_id::<ExternType>(t) }.is_null() {
            s.push_str("table");
        } else {
            s.push_str("external type");
        }

        s.push_str(" '");
        s.push_str(&to_string_type_id(e.table()));
        s.push_str("'.  ");
        s.push_str(&candidates_suggestion);
        s.push('?');

        s
    }
}
