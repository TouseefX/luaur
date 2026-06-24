use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::type_aliases::type_pack_id::TypePackId;

impl IterativeTypeVisitor {
    pub fn visit_type_pack_id_blocked_type_pack(
        &mut self,
        tp: TypePackId,
        _btp: &BlockedTypePack,
    ) -> bool {
        self.visit_type_pack_id(tp)
    }
}
