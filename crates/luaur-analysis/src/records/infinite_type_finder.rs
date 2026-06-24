use crate::records::constraint_solver::ConstraintSolver;
use crate::records::instantiation_signature::InstantiationSignature;
use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::records::scope::Scope;
use core::ptr::NonNull;

#[derive(Debug, Clone)]
pub struct InfiniteTypeFinder {
    pub base: IterativeTypeVisitor,
    pub solver: *mut ConstraintSolver,
    pub signature: InstantiationSignature,
    pub scope: NonNull<Scope>,
    pub found_infinite_type: bool,
}
