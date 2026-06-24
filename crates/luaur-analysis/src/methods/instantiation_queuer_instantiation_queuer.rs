use crate::records::constraint_solver::ConstraintSolver;
use crate::records::instantiation_queuer::InstantiationQueuer;
use crate::records::scope::Scope;
use core::ptr::NonNull;
use luaur_ast::records::location::Location;

impl InstantiationQueuer {
    pub fn instantiation_queuer(
        scope: NonNull<Scope>,
        location: &Location,
        solver: *mut ConstraintSolver,
    ) -> Self {
        let mut visitor = InstantiationQueuer {
            base: crate::records::iterative_type_visitor::IterativeTypeVisitor {
                seen: crate::type_aliases::seen_set_iterative_type_visitor::SeenSet::default(),
                work_queue: alloc::vec::Vec::new(),
                parent_cursor: -1,
                work_cursor: 0,
                visitor_name: alloc::string::String::from("InstantiationQueuer"),
                skip_bound_types: true,
                visit_once: true,
            },
            solver,
            scope,
            location: *location,
        };
        visitor
            .base
            .iterative_type_visitor_string_bool_bool("InstantiationQueuer", true, true);
        visitor
    }
}
