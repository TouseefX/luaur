use crate::functions::get_code_gen_context::get_code_gen_context;
use crate::type_aliases::lua_state::lua_State;
use luaur_vm::records::lua_state::lua_State as LuaStateInternal;

#[inline]
pub fn is_native_execution_enabled(L: *mut lua_State) -> bool {
    // SAFETY: Accessing L->global->ecb.enter requires L to be a valid pointer.
    // The C++ implementation checks if the context exists and if the enter callback
    // matches the 'onEnter' function pointer.
    unsafe {
        let context = get_code_gen_context(L);
        if context.is_null() {
            return false;
        }

        let l_internal = L as *mut LuaStateInternal;
        if l_internal.is_null() {
            return false;
        }

        let global = (*l_internal).global;
        if global.is_null() {
            return false;
        }

        // The C++ source compares the function pointer directly.
        // We assume 'on_enter' is the Rust translation of the 'onEnter' symbol.
        // We compare the function pointer addresses.
        let current_enter = (*global).ecb.enter;

        // We need to compare against the address of the 'on_enter' function.
        // Import it from the crate where it is defined.
        let on_enter_ptr = crate::functions::on_enter::on_enter as *const ();

        if let Some(enter_fn) = current_enter {
            let enter_fn_ptr = enter_fn as *const ();
            enter_fn_ptr == on_enter_ptr
        } else {
            false
        }
    }
}
