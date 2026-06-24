use crate::enums::type_function_instance_state::TypeFunctionInstanceState;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reducer::TypeFunctionReducer;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeFunctionReducer {
    pub fn get_state_type_id(&self, ty: TypeId) -> TypeFunctionInstanceState {
        let tfit = unsafe { get_type_id::<TypeFunctionInstanceType>(ty) };
        LUAU_ASSERT!(!tfit.is_null());
        unsafe { (*tfit).state }
    }
}
