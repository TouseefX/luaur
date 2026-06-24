use crate::records::base_code_gen_context::BaseCodeGenContext;
use crate::type_aliases::lua_state::lua_State;

#[inline]
pub fn get_code_gen_context(L: *mut lua_State) -> *mut BaseCodeGenContext {
    // SAFETY: This mirrors the C++ implementation:
    // return static_cast<BaseCodeGenContext*>(L->global->ecb.context);
    // Caller must ensure `L` is a valid lua_State pointer with a valid `global`
    // and that `global->ecb.context` points to a BaseCodeGenContext (or is null).
    unsafe {
        if L.is_null() {
            return core::ptr::null_mut();
        }

        let global = (*L).global;
        if global.is_null() {
            return core::ptr::null_mut();
        }

        let ctx = (*global).ecb.context;
        ctx as *mut BaseCodeGenContext
    }
}
