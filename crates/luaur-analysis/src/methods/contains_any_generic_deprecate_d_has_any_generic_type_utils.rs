use crate::records::contains_any_generic_deprecated::ContainsAnyGenericDeprecated;
use crate::type_aliases::type_id::TypeId;

impl ContainsAnyGenericDeprecated {
    pub fn has_any_generic_type_id(&mut self, ty: TypeId) -> bool {
        // C++ `ContainsAnyGeneric_DEPRECATED::hasAnyGeneric(TypeId)`:
        // construct, `traverse(ty)`, return `found`.
        ContainsAnyGenericDeprecated::has_any_generic(ty)
    }
}
