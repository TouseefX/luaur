use crate::records::constraint_solver::ConstraintSolver;
use crate::records::scope::Scope;
use crate::records::type_once_visitor::TypeOnceVisitor;
use core::ptr::NonNull;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone)]
pub struct InstantiationQueuerDeprecated {
    pub base: TypeOnceVisitor,
    pub solver: *mut ConstraintSolver,
    pub scope: NonNull<Scope>,
    pub location: Location,
}
