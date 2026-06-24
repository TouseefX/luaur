use crate::enums::type_function_instance_state::TypeFunctionInstanceState;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::blocked_type::BlockedType;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::type_aliases::type_id::TypeId;

pub fn is_blocked(ty: TypeId) -> bool {
    let ty = unsafe { follow_type_id(ty) };

    unsafe {
        let tfit = get_type_id::<TypeFunctionInstanceType>(ty);
        if !tfit.is_null() && (*tfit).state == TypeFunctionInstanceState::Unsolved {
            return true;
        }

        !get_type_id::<BlockedType>(ty).is_null()
            || !get_type_id::<PendingExpansionType>(ty).is_null()
    }
}
