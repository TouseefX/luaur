use crate::records::constraint_solver::ConstraintSolver;
use crate::records::time_limit_error::TimeLimitError;

impl ConstraintSolver {
    pub fn constraint_solver_throw_time_limit_error(&self) {
        panic!("{}", TimeLimitError::time_limit_error_time_limit_error(""));
    }
}
