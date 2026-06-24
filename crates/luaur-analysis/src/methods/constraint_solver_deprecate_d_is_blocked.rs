use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;

impl ConstraintSolver {
    pub fn deprecate_d_is_blocked(&self, constraint: *const Constraint) -> bool {
        let blocked_it = self.deprecated_blocked_constraints.get(&constraint);
        blocked_it.map_or(false, |blocked| *blocked > 0)
    }
}
