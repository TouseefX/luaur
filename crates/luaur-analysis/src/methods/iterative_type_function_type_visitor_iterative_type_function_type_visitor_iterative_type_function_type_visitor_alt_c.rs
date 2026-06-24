use crate::records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor;
use crate::type_aliases::seen_set_iterative_type_function_type_visitor::SeenSet;
use alloc::string::String;

impl IterativeTypeFunctionTypeVisitor {
    pub fn iterative_type_function_type_visitor_string(visitor_name: String) -> Self {
        Self::iterative_type_function_type_visitor_string_seen_set_bool(
            visitor_name,
            SeenSet::new(core::ptr::null()),
            /*visitOnce*/ true,
        )
    }
}
