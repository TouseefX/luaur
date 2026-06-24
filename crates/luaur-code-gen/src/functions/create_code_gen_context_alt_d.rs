use crate::functions::initialize_execution_callbacks::initialize_execution_callbacks;
use crate::records::shared_code_gen_context::SharedCodeGenContext;
use crate::type_aliases::lua_state::lua_State;

pub fn create(l: *mut lua_State, code_gen_context: *mut SharedCodeGenContext) {
    // SAFETY: code_gen_context is a pointer to a SharedCodeGenContext,
    // which inherits from BaseCodeGenContext. The initialize_execution_callbacks
    // function expects a pointer to the base class.
    let base_context = unsafe { &mut (*code_gen_context).base };
    initialize_execution_callbacks(l, base_context as *mut _);
}

#[allow(non_snake_case)]
pub fn create_lua_state_shared_code_gen_context(
    l: *mut lua_State,
    code_gen_context: *mut SharedCodeGenContext,
) {
    create(l, code_gen_context);
}
