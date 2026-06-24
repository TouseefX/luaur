use crate::records::extern_type::ExternType;
use crate::records::generic_type_finder::GenericTypeFinder;
use crate::type_aliases::type_id::TypeId;

impl GenericTypeFinder {
    pub fn visit_type_id_luau_mut(&mut self, _ty: TypeId, _etv: &ExternType) -> bool {
        false
    }
}
