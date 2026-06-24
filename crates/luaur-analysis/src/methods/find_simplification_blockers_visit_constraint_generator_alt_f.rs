use crate::records::extern_type::ExternType;
use crate::records::find_simplification_blockers::FindSimplificationBlockers;
use crate::type_aliases::type_id::TypeId;

impl FindSimplificationBlockers {
    pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _etv: &ExternType) -> bool {
        false
    }
}
