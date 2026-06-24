use crate::records::extern_type::ExternType;
use crate::records::generic_counter::GenericCounter;
use crate::type_aliases::type_id::TypeId;

impl GenericCounter {
    pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _et: &ExternType) -> bool {
        false
    }
}
