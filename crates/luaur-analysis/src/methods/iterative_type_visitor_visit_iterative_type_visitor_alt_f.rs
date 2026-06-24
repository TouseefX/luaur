use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::records::primitive_type::PrimitiveType;
use crate::type_aliases::type_id::TypeId;

impl IterativeTypeVisitor {
    pub fn visit_type_id_primitive_type(&mut self, ty: TypeId, _ptv: &PrimitiveType) -> bool {
        self.visit_type_id(ty)
    }
}
