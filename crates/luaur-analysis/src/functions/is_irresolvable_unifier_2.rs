use crate::enums::type_function_instance_state::TypeFunctionInstanceState;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::blocked_type::BlockedType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::type_aliases::type_id::TypeId;

pub fn is_irresolvable(ty: TypeId) -> bool {
    unsafe {
        let tfit = get_type_id::<TypeFunctionInstanceType>(ty);
        if !tfit.is_null() && (*tfit).state != TypeFunctionInstanceState::Unsolved {
            return false;
        }
        !get_type_id::<BlockedType>(ty).is_null() || !tfit.is_null()
    }
}
