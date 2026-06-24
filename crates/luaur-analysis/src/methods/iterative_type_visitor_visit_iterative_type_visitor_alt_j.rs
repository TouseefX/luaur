use crate::records::extern_type::ExternType;
use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::type_aliases::type_id::TypeId;

impl IterativeTypeVisitor {
    pub fn visit_type_id_extern_type(&mut self, ty: TypeId, _etv: &ExternType) -> bool {
        self.visit_type_id(ty)
    }
}
