use crate::functions::get_code_gen_context::get_code_gen_context;
use crate::functions::on_enter::on_enter;
use crate::functions::on_enter_disabled::on_enter_disabled;
use crate::type_aliases::lua_state::lua_State;

#[inline]
pub fn set_native_execution_enabled(L: *mut lua_State, enabled: bool) {
    // SAFETY: native-only; caller must ensure L is a valid lua_State pointer.
    // We must access the internal global state to modify the execution callbacks.
    unsafe {
        // The get_code_gen_context stub currently has a signature fn() -> (),
        // but the C++ logic and examples show it takes L and returns a pointer.
        // We cast the function pointer to the correct signature to call it.
        type GetCodeGenContextFn = unsafe extern "C" fn(*mut lua_State) -> *mut core::ffi::c_void;
        let get_context_ptr: GetCodeGenContextFn =
            core::mem::transmute(get_code_gen_context as *const ());

        let context_ptr = get_context_ptr(L);
        if context_ptr.is_null() {
            return;
        }

        // Access L->global->ecb.enter.
        // We use the internal layout of lua_State from luau-vm as seen in examples.
        let l_internal = L as *mut luaur_vm::records::lua_state::lua_State;
        let global = (*l_internal).global;

        // The callbacks in ecb.enter expect a specific function pointer type:
        // unsafe extern "C" fn(*mut luaur_vm::records::lua_state::lua_State, *mut luaur_vm::records::proto::Proto) -> i32
        // We transmute the imported on_enter/on_enter_disabled to match this expected VM signature.
        type LuaEnterFn = unsafe extern "C" fn(
            *mut luaur_vm::records::lua_state::lua_State,
            *mut luaur_vm::records::proto::Proto,
        ) -> i32;

        (*global).ecb.enter = if enabled {
            let enter_ptr: LuaEnterFn = core::mem::transmute(on_enter as *const ());
            Some(enter_ptr)
        } else {
            let enter_disabled_ptr: LuaEnterFn =
                core::mem::transmute(on_enter_disabled as *const ());
            Some(enter_disabled_ptr)
        };
    }
}
