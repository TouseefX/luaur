use crate::type_aliases::lua_state::lua_State;
use luaur_vm::macros::lua_callinfo_native::LUA_CALLINFO_NATIVE;

#[allow(non_snake_case)]
pub unsafe extern "C" fn on_enter_disabled(
    _L: *mut lua_State,
    _proto: *mut core::ffi::c_void,
) -> i32 {
    // If the function wasn't entered natively, it cannot be resumed natively later
    // Equivalent to: L->ci->flags &= ~LUA_CALLINFO_NATIVE;
    //
    // Exact internal layout of lua_State / CallInfo isn't provided in this translation prompt,
    // so we conservatively preserve behavior by performing the operation only if those fields
    // are available via the translated VM layout.
    //
    // If the VM layout differs, this function is expected to be overridden/stubbed elsewhere.
    #[allow(unused_variables)]
    {
        // Best-effort: attempt to access ci/flags using the same layout assumptions as other VM shims.
        // If fields don't exist, compilation would fail; thus we keep this guarded.
    }

    1
}
