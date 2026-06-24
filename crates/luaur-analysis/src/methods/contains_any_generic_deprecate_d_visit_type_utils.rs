use crate::records::contains_any_generic_deprecated::ContainsAnyGenericDeprecated;
use crate::records::extern_type::ExternType;
use crate::type_aliases::type_id::TypeId;

impl ContainsAnyGenericDeprecated {
    pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _ext: &ExternType) -> bool {
        false
    }
}
