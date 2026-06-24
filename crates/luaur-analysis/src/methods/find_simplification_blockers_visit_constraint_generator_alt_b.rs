use crate::records::blocked_type::BlockedType;
use crate::records::find_simplification_blockers::FindSimplificationBlockers;
use crate::type_aliases::type_id::TypeId;

impl FindSimplificationBlockers {
    pub fn visit_type_id_blocked_type(&mut self, _ty: TypeId, _btv: &BlockedType) -> bool {
        self.found = true;
        false
    }
}
