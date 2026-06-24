use crate::records::normalized_string_type::NormalizedStringType;

pub fn normalized_string_type_reset_to_string(normalized_string_type: &mut NormalizedStringType) {
    normalized_string_type.isCofinite = true;
    normalized_string_type.singletons.clear();
}
