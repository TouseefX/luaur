use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::type_aliases::bound_type_pack::BoundTypePack;
use crate::type_aliases::type_pack_id::TypePackId;

impl IterativeTypeVisitor {
    pub fn visit_type_pack_id_bound_type_pack(
        &mut self,
        tp: TypePackId,
        _btp: &BoundTypePack,
    ) -> bool {
        self.visit_type_pack_id(tp)
    }
}
