use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::records::type_pack::TypePack;
use crate::type_aliases::type_pack_id::TypePackId;

impl IterativeTypeVisitor {
    pub fn visit_type_pack_id_type_pack(&mut self, tp: TypePackId, _pack: &TypePack) -> bool {
        self.visit_type_pack_id(tp)
    }
}
