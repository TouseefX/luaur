use crate::records::generic_type_pack::GenericTypePack;
use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::type_aliases::type_pack_id::TypePackId;

impl IterativeTypeVisitor {
    pub fn visit_type_pack_id_generic_type_pack(
        &mut self,
        tp: TypePackId,
        _gtp: &GenericTypePack,
    ) -> bool {
        self.visit_type_pack_id(tp)
    }
}
