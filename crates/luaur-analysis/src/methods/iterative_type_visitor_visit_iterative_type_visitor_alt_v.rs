use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::type_aliases::type_pack_id::TypePackId;

impl IterativeTypeVisitor {
    pub fn visit_type_pack_id(&mut self, _tp: TypePackId) -> bool {
        true
    }
}
