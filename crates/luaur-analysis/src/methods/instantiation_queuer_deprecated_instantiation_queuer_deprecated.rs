use crate::records::constraint_solver::ConstraintSolver;
use crate::records::instantiation_queuer_deprecated::InstantiationQueuerDeprecated;
use crate::records::scope::Scope;
use crate::records::type_once_visitor::TypeOnceVisitor;
use alloc::string::String;
use core::ptr::NonNull;
use luaur_ast::records::location::Location;

impl InstantiationQueuerDeprecated {
    pub fn instantiation_queuer_deprecated_instantiation_queuer_deprecated(
        scope: NonNull<Scope>,
        location: &Location,
        solver: *mut ConstraintSolver,
    ) -> Self {
        Self {
            base: TypeOnceVisitor::type_once_visitor(String::from("InstantiationQueuer"), true),
            solver,
            scope,
            location: *location,
        }
    }
}
