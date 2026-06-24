use crate::records::blocked_type::BlockedType;
use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::type_aliases::type_id::TypeId;

impl IterativeTypeVisitor {
    pub fn visit_type_id_blocked_type(&mut self, ty: TypeId, _btv: &BlockedType) -> bool {
        self.visit_type_id(ty)
    }
}
