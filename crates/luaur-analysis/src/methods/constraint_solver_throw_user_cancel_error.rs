use crate::records::constraint_solver::ConstraintSolver;
use crate::records::user_cancel_error::UserCancelError;

impl ConstraintSolver {
    pub fn constraint_solver_throw_user_cancel_error(&self) {
        panic!("{:?}", UserCancelError::new("".to_string()));
    }
}
