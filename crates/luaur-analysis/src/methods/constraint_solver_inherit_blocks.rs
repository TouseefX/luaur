use crate::records::constraint::Constraint;
use crate::records::constraint_graph::ConstraintGraph;
use crate::records::constraint_solver::ConstraintSolver;
use crate::type_aliases::blocked_constraint_id::BlockedConstraintId;
use crate::FFlag;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl ConstraintSolver {
    pub fn inherit_blocks(&mut self, source: *const Constraint, addition: *const Constraint) {
        if FFlag::LuauConstraintGraph.get() {
            unsafe {
                (*self.cgraph).inherit_blocks(
                    crate::type_aliases::constraint_vertex::ConstraintVertex::V2(source),
                    crate::type_aliases::constraint_vertex::ConstraintVertex::V2(addition),
                )
            }
        } else {
            // Anything that is blocked on this constraint must also be blocked on our
            // synthesized constraints.
            let blocked_constraints: alloc::vec::Vec<*const Constraint> = match self
                .deprecated_blocked
                .get(&BlockedConstraintId::V2(source))
            {
                Some(blocked_set) => blocked_set.iter().copied().collect(),
                None => alloc::vec::Vec::new(),
            };
            for blocked_constraint in blocked_constraints {
                self.block_not_null_constraint_not_null_constraint(addition, blocked_constraint);
            }
        }
    }
}
