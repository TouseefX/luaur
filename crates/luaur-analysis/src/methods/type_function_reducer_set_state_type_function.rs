use crate::enums::type_function_instance_state::TypeFunctionInstanceState;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reducer::TypeFunctionReducer;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeFunctionReducer {
    pub fn set_state_type_id_type_function_instance_state(
        &self,
        ty: TypeId,
        state: TypeFunctionInstanceState,
    ) {
        if unsafe { (*ty).owning_arena != (*self.ctx.as_ptr()).arena.as_ptr() } {
            return;
        }

        let tfit = unsafe { get_mutable_type_id::<TypeFunctionInstanceType>(ty) };
        LUAU_ASSERT!(!tfit.is_null());
        unsafe {
            (*tfit).state = state;
        }
    }
}
