use crate::records::freeze_type_function_types::FreezeTypeFunctionTypes;
use crate::records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor;

impl FreezeTypeFunctionTypes {
    pub fn freeze_type_function_types_freeze_type_function_types(&mut self) {
        self.base = IterativeTypeFunctionTypeVisitor::iterative_type_function_type_visitor_string(
            "FreezeTypeFunctionTypes".to_string(),
        );
    }
}
