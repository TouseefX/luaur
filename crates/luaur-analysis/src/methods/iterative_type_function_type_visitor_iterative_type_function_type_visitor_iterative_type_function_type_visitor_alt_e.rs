use crate::records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor;
use crate::type_aliases::seen_set_iterative_type_function_type_visitor::SeenSet;
use alloc::string::String;
use alloc::vec::Vec;

impl IterativeTypeFunctionTypeVisitor {
    pub fn iterative_type_function_type_visitor_string_seen_set_bool(
        visitor_name: String,
        seen: SeenSet,
        visit_once: bool,
    ) -> Self {
        // Skip the first few doublings.  Almost all visits require less than 32 steps.
        let mut work_queue = Vec::new();
        work_queue.reserve(32);

        IterativeTypeFunctionTypeVisitor {
            seen,
            work_queue,
            parent_cursor: -1,
            work_cursor: 0,
            visitor_name,
            visit_once,
        }
    }
}
