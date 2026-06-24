use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::type_aliases::blocked_constraint_id::BlockedConstraintId;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl ConstraintSolver {
    pub fn deprecate_d_block(
        &mut self,
        target: BlockedConstraintId,
        constraint: *const Constraint,
    ) -> bool {
        let block_vec = self
            .deprecated_blocked
            .entry(target)
            .or_insert_with(|| DenseHashSet::new(core::ptr::null()));

        if block_vec.find(&constraint).is_some() {
            return false;
        }

        block_vec.insert(constraint);

        let count = self
            .deprecated_blocked_constraints
            .entry(constraint)
            .or_insert(0);
        *count += 1;

        true
    }
}
