use crate::records::find_function_type_in::FindFunctionTypeIn;
use crate::records::iterative_type_visitor::IterativeTypeVisitor;

impl FindFunctionTypeIn {
    pub fn find_function_type_in(number_of_lambda_parameters: i32) -> Self {
        let mut visitor = FindFunctionTypeIn {
            base: IterativeTypeVisitor {
                seen: luaur_common::records::dense_hash_set::DenseHashSet::new(
                    core::ptr::null_mut(),
                ),
                work_queue: alloc::vec::Vec::new(),
                parent_cursor: -1,
                work_cursor: 0,
                visitor_name: alloc::string::String::from("FindFunctionTypeIn"),
                skip_bound_types: true,
                visit_once: true,
            },
            number_of_lambda_parameters,
            candidate: core::ptr::null(),
        };
        visitor
            .base
            .iterative_type_visitor_string_bool_bool("FindFunctionTypeIn", true, true);
        visitor
    }
}
