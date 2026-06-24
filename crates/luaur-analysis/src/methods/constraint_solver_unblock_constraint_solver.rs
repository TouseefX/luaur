use crate::records::constraint_solver::ConstraintSolver;
use crate::type_aliases::blocked_constraint_id::BlockedConstraintId;
use crate::type_aliases::bound_type::BoundType;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::FFlag;

impl ConstraintSolver {
    pub fn unblock_type_id_location(&mut self, ty: TypeId, location: Location) {
        let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(core::ptr::null());
        let mut progressed = ty;

        loop {
            if seen.contains(&progressed) {
                self.ice_reporter.ice_string_location(
                    "ConstraintSolver::unblock encountered a self-bound type!",
                    &location,
                );
            }
            seen.insert(progressed);

            if let Some(logger) = unsafe { self.logger.as_mut() } {
                logger.pop_block_type_id(progressed);
            }

            if !FFlag::LuauConstraintGraph.get() {
                self.deprecate_d_unblock_(BlockedConstraintId::V0(progressed));
            }

            let bound =
                unsafe { crate::functions::get_type_alt_j::get_type_id::<BoundType>(progressed) };
            if bound.is_null() {
                break;
            }

            progressed = unsafe { (*bound).boundTo };
        }

        if FFlag::LuauConstraintGraph.get() {
            unsafe {
                (*self.cgraph).unblock_type_or_pack_type_id(ty);
            }
        }
    }
}
