use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

impl IterativeTypeVisitor {
    pub fn visit_type_id_union_type(&mut self, ty: TypeId, _utv: &UnionType) -> bool {
        self.visit_type_id(ty)
    }
}
