use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::type_aliases::type_id::TypeId;

impl IterativeTypeVisitor {
    pub fn visit_type_id_type_function_instance_type(
        &mut self,
        ty: TypeId,
        _tfit: &TypeFunctionInstanceType,
    ) -> bool {
        self.visit_type_id(ty)
    }
}
