use crate::records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor;
use crate::records::type_function_table_type::TypeFunctionTableType;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;

impl IterativeTypeFunctionTypeVisitor {
    pub fn visit_type_function_type_id_type_function_table_type(
        &mut self,
        ty: TypeFunctionTypeId,
        _tftt: &TypeFunctionTableType,
    ) -> bool {
        self.visit_type_function_type_id(ty)
    }
}
