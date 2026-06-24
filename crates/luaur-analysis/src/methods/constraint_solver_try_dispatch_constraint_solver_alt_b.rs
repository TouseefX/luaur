use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::subtype_constraint::SubtypeConstraint;
use crate::type_aliases::constraint_v::ConstraintV;
use core::ptr::NonNull;

impl ConstraintSolver {
    pub fn try_dispatch_subtype_constraint_not_null_constraint(
        &mut self,
        c: &SubtypeConstraint,
        constraint: *const Constraint,
    ) -> bool {
        if self.is_blocked_type_id(c.sub_type) {
            return self.block_type_id_not_null_constraint(c.sub_type, constraint);
        } else if self.is_blocked_type_id(c.super_type) {
            return self.block_type_id_not_null_constraint(c.super_type, constraint);
        }

        self.constraint_solver_unify(constraint, c.sub_type, c.super_type);

        true
    }
}
