use crate::records::find_simplification_blockers::FindSimplificationBlockers;
use crate::records::free_type::FreeType;
use crate::type_aliases::type_id::TypeId;

impl FindSimplificationBlockers {
    pub fn visit_type_id_free_type(&mut self, _ty: TypeId, _ftv: &FreeType) -> bool {
        self.found = true;
        false
    }
}
