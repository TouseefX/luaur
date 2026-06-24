use crate::records::extern_type::ExternType;
use crate::records::find_refinement_blockers::FindRefinementBlockers;
use crate::type_aliases::type_id::TypeId;

impl FindRefinementBlockers {
    pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _extern: &ExternType) -> bool {
        false
    }
}
