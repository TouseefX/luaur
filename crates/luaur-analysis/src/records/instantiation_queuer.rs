use crate::records::constraint_solver::ConstraintSolver;
use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::records::scope::Scope;
use core::ptr::NonNull;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone)]
pub struct InstantiationQueuer {
    pub base: IterativeTypeVisitor,
    pub solver: *mut ConstraintSolver,
    pub scope: NonNull<Scope>,
    pub location: Location,
}
