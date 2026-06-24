use crate::records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor;
use crate::records::type_function_type::TypeFunctionType;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use alloc::string::String;

#[derive(Debug, Clone)]
pub struct FreezeTypeFunctionTypes {
    pub base: IterativeTypeFunctionTypeVisitor,
}

impl FreezeTypeFunctionTypes {
    pub fn new() -> Self {
        Self {
            base: IterativeTypeFunctionTypeVisitor::iterative_type_function_type_visitor_string(
                String::from("FreezeTypeFunctionTypes"),
            ),
        }
    }
}

impl crate::records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor {
    pub fn visit_type_function_type_id(&mut self, ty: TypeFunctionTypeId) -> bool {
        unsafe {
            (*(ty as *mut TypeFunctionType)).frozen = true;
        }
        true
    }
}
