use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::records::singleton_type::SingletonType;
use crate::type_aliases::type_id::TypeId;

impl IterativeTypeVisitor {
    pub fn visit_type_id_singleton_type(&mut self, ty: TypeId, _stv: &SingletonType) -> bool {
        self.visit_type_id(ty)
    }
}
