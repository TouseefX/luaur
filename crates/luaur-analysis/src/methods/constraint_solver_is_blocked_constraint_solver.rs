use crate::enums::type_function_instance_state::TypeFunctionInstanceState;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::type_aliases::type_id::TypeId;

impl ConstraintSolver {
    pub fn is_blocked_type_id(&self, ty: TypeId) -> bool {
        // FIXME CLI-180636: Eventually this should use the same logic as
        // `SubtypingUnifier`, which is that blocked types are only based
        // on their type and any additional state, rather than looking at
        // `uninhabitedTypeFunctions`.
        let ty = unsafe { follow_type_id(ty) };

        let tfit = unsafe { get_type_id::<TypeFunctionInstanceType>(ty) };
        if !tfit.is_null() {
            if unsafe { (*tfit).state } != TypeFunctionInstanceState::Unsolved {
                return false;
            }

            return !self
                .uninhabited_type_functions
                .contains(&(ty as *const core::ffi::c_void));
        }

        !unsafe { get_type_id::<BlockedType>(ty) }.is_null()
            || !unsafe { get_type_id::<PendingExpansionType>(ty) }.is_null()
    }
}
