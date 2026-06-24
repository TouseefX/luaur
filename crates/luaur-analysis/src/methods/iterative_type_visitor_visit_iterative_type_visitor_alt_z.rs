use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::type_aliases::error_type_pack::ErrorTypePack;
use crate::type_aliases::type_pack_id::TypePackId;

impl IterativeTypeVisitor {
    pub fn visit_type_pack_id_error_type_pack(
        &mut self,
        tp: TypePackId,
        _etp: &ErrorTypePack,
    ) -> bool {
        self.visit_type_pack_id(tp)
    }
}
