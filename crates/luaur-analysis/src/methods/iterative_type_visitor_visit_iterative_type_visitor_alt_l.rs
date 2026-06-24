use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::records::no_refine_type::NoRefineType;
use crate::type_aliases::type_id::TypeId;

impl IterativeTypeVisitor {
    pub fn visit_type_id_no_refine_type(&mut self, ty: TypeId, _nrt: &NoRefineType) -> bool {
        self.visit_type_id(ty)
    }
}
