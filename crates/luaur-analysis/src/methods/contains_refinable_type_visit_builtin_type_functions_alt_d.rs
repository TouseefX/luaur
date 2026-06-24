use crate::records::contains_refinable_type::ContainsRefinableType;
use crate::records::metatable_type::MetatableType;
use crate::type_aliases::type_id::TypeId;

impl ContainsRefinableType {
    pub fn visit_type_id_metatable_type(
        &mut self,
        _ty: TypeId,
        _metatable: &MetatableType,
    ) -> bool {
        !self.found
    }
}
