use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::records::negation_type::NegationType;
use crate::type_aliases::type_id::TypeId;

impl IterativeTypeVisitor {
    pub fn visit_type_id_negation_type(&mut self, ty: TypeId, _ntv: &NegationType) -> bool {
        self.visit_type_id(ty)
    }
}
