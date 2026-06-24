use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::type_aliases::type_pack_id::TypePackId;

impl ConstraintSolver {
    pub fn block_type_pack_id_not_null_constraint(
        &mut self,
        target: TypePackId,
        constraint: *const Constraint,
    ) -> bool {
        let target = unsafe { follow_type_pack_id(target) };
        let new_block = if luaur_common::FFlag::LuauConstraintGraph.get() {
            unsafe {
                (*self.cgraph).add_dependency_of_constraint_vertex_constraint_vertex(
                    crate::type_aliases::constraint_vertex::ConstraintVertex::V1(target),
                    crate::type_aliases::constraint_vertex::ConstraintVertex::V2(constraint),
                )
            }
        } else {
            self.deprecate_d_block(
                crate::type_aliases::blocked_constraint_id::BlockedConstraintId::V1(target),
                constraint,
            )
        };

        if new_block {
            if let Some(logger) = unsafe { self.logger.as_mut() } {
                logger.push_block_not_null_constraint_type_pack_id(constraint, target);
            }
        }
        false
    }
}
