use crate::records::contains_refinable_type::ContainsRefinableType;
use crate::records::function_type::FunctionType;
use crate::type_aliases::type_id::TypeId;

impl ContainsRefinableType {
    pub fn visit_type_id_function_type(
        &mut self,
        _ty: TypeId,
        _function_type: &FunctionType,
    ) -> bool {
        !self.found
    }
}
