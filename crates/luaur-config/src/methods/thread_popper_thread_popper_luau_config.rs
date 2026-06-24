use crate::records::thread_popper::ThreadPopper;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_vm::type_aliases::lua_state::lua_State;

impl ThreadPopper {
    pub fn thread_popper_lua_state(l: *mut lua_State) -> Self {
        LUAU_ASSERT!(!l.is_null());
        ThreadPopper { L: l }
    }
}
