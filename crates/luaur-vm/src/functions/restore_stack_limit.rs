use crate::functions::lua_d_realloc_ci::lua_d_realloc_ci;
use crate::macros::cast_int::cast_int;
use crate::macros::extra_stack::EXTRA_STACK;
use crate::macros::luai_maxcalls::LUAI_MAXCALLS;
use crate::type_aliases::lua_state::lua_State;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

#[allow(dead_code)]
pub unsafe fn restore_stack_limit(L: *mut lua_State) {
    LUAU_ASSERT!(
        (*L).stack_last.offset_from((*L).stack) == ((*L).stacksize - EXTRA_STACK) as isize
    );
    if (*L).size_ci > LUAI_MAXCALLS {
        let inuse = cast_int!((*L).ci.offset_from((*L).base_ci));
        if inuse + 1 < LUAI_MAXCALLS {
            // The dependency lua_d_realloc_ci is currently a stub with 0 arguments.
            // We must call it with the arguments required by the C++ source (L, LUAI_MAXCALLS).
            // The coordinator will update the stub signature to match this call.
            let func: unsafe fn(*mut lua_State, core::ffi::c_int) =
                core::mem::transmute(lua_d_realloc_ci as *const ());
            func(L, LUAI_MAXCALLS);
        }
    }
}
