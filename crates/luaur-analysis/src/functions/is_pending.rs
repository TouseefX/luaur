use crate::enums::type_function_instance_state::TypeFunctionInstanceState;
use crate::functions::get_constraint::get_constraint;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::type_aliases::type_id::TypeId;

pub fn is_pending(ty: TypeId, solver: *mut ConstraintSolver) -> bool {
    unsafe {
        let tfit = get_type_id::<TypeFunctionInstanceType>(ty);
        if !tfit.is_null() {
            if (*tfit).state == TypeFunctionInstanceState::Unsolved {
                return true;
            }
        }

        let blocked = get_type_id::<BlockedType>(ty);
        if !blocked.is_null() {
            return true;
        }

        let pending = get_type_id::<PendingExpansionType>(ty);
        if !pending.is_null() {
            return true;
        }

        if !solver.is_null() {
            return (*solver).has_unresolved_constraints(ty);
        }

        false
    }
}
