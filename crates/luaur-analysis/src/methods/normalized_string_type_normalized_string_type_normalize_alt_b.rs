use crate::records::normalized_string_type::NormalizedStringType;
use crate::type_aliases::type_id::TypeId;

impl NormalizedStringType {
    pub fn normalized_string_type_bool_map_string_type_id(
        &mut self,
        is_cofinite: bool,
        singletons: alloc::collections::BTreeMap<alloc::string::String, TypeId>,
    ) {
        self.isCofinite = is_cofinite;
        self.singletons = singletons;
    }
}
