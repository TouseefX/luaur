use crate::functions::conformance_interrupt_inspection_hook::conformance_interrupt_inspection_hook;
use luaur_vm::functions::lua_getinfo::lua_getinfo;
use luaur_vm::functions::luau_callhook::luau_callhook;
use luaur_vm::records::lua_debug::LuaDebug;
use luaur_vm::records::lua_state::lua_State;

pub unsafe extern "C" fn conformance_interrupt_inspection_yield(l: *mut lua_State) -> bool {
    let mut ar: LuaDebug = core::mem::zeroed();
    assert_ne!(0, lua_getinfo(l, 0, c"nsl".as_ptr(), &mut ar));

    luau_callhook(
        l,
        Some(conformance_interrupt_inspection_hook),
        core::ptr::null_mut(),
    );

    false
}
