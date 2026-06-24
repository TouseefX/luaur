use crate::records::any_type::AnyType;
use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::type_aliases::type_id::TypeId;

impl IterativeTypeVisitor {
    pub fn visit_type_id_any_type(&mut self, ty: TypeId, _atv: &AnyType) -> bool {
        self.visit_type_id(ty)
    }
}
