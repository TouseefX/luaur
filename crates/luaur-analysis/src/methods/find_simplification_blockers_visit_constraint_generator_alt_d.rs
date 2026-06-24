use crate::records::find_simplification_blockers::FindSimplificationBlockers;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::type_aliases::type_id::TypeId;

impl FindSimplificationBlockers {
    pub fn visit_type_id_pending_expansion_type(
        &mut self,
        _ty: TypeId,
        _petv: &PendingExpansionType,
    ) -> bool {
        self.found = true;
        false
    }
}
