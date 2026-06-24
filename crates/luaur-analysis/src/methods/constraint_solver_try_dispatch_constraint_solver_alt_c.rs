use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::pack_subtype_constraint::PackSubtypeConstraint;
use crate::type_aliases::constraint_v::ConstraintV;
use core::ptr::NonNull;

impl ConstraintSolver {
    pub fn try_dispatch_pack_subtype_constraint_not_null_constraint(
        &mut self,
        c: &PackSubtypeConstraint,
        constraint: *const Constraint,
    ) -> bool {
        if self.is_blocked_type_pack_id(c.sub_pack) {
            return self.block_type_pack_id_not_null_constraint(c.sub_pack, constraint);
        } else if self.is_blocked_type_pack_id(c.super_pack) {
            return self.block_type_pack_id_not_null_constraint(c.super_pack, constraint);
        }

        self.constraint_solver_unify(constraint, c.sub_pack, c.super_pack);

        true
    }
}
