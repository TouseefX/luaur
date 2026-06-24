use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::type_aliases::bound_type::BoundType;
use crate::type_aliases::type_id::TypeId;

impl IterativeTypeVisitor {
    pub fn visit_type_id_bound_type(&mut self, ty: TypeId, _btv: &BoundType) -> bool {
        self.visit_type_id(ty)
    }
}
