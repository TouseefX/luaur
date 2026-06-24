use crate::records::constraint_solver::ConstraintSolver;
use crate::type_aliases::blocked_constraint_id::BlockedConstraintId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::location::Location;
use luaur_common::FFlag;

impl ConstraintSolver {
    pub fn unblock_type_pack_id_location(&mut self, tp: TypePackId, _location: Location) {
        if let Some(logger) = unsafe { self.logger.as_mut() } {
            logger.pop_block_type_pack_id(tp);
        }

        if FFlag::LuauConstraintGraph.get() {
            unsafe {
                (*self.cgraph).unblock_type_or_pack_type_pack_id(tp);
            }
        } else {
            self.deprecate_d_unblock_(BlockedConstraintId::V1(tp));
        }
    }
}
