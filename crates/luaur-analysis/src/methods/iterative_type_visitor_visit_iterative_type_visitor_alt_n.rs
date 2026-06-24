use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::records::never_type::NeverType;
use crate::type_aliases::type_id::TypeId;

impl IterativeTypeVisitor {
    pub fn visit_type_id_never_type(&mut self, ty: TypeId, _ntv: &NeverType) -> bool {
        self.visit_type_id(ty)
    }
}
