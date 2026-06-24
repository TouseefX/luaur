use crate::records::constraint_solver::ConstraintSolver;
use crate::records::infinite_type_finder::InfiniteTypeFinder;
use crate::records::instantiation_signature::InstantiationSignature;
use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::records::scope::Scope;
use core::ptr::NonNull;

impl InfiniteTypeFinder {
    pub fn infinite_type_finder_infinite_type_finder(
        solver: *mut ConstraintSolver,
        signature: &InstantiationSignature,
        scope: NonNull<Scope>,
    ) -> Self {
        let mut visitor = InfiniteTypeFinder {
            base: IterativeTypeVisitor {
                seen: luaur_common::records::dense_hash_set::DenseHashSet::new(core::ptr::null_mut()),
                work_queue: alloc::vec::Vec::new(),
                parent_cursor: -1,
                work_cursor: 0,
                visitor_name: alloc::string::String::from("InfiniteTypeFinder"),
                skip_bound_types: true,
                visit_once: true,
            },
            solver,
            signature: signature.clone(),
            scope,
            found_infinite_type: false,
        };
        visitor
            .base
            .iterative_type_visitor_string_bool_bool("InfiniteTypeFinder", true, true);
        visitor
    }
}
