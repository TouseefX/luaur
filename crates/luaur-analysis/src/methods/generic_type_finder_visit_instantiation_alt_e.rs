use crate::records::generic_type::GenericType;
use crate::records::generic_type_finder::GenericTypeFinder;
use crate::type_aliases::type_id::TypeId;

impl GenericTypeFinder {
    pub fn visit_type_id_luau(&mut self, _ty: TypeId, _gtv: &GenericType) -> bool {
        self.found = true;
        false
    }
}
