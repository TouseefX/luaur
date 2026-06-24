use luaur_vm::functions::lua_getinfo::lua_getinfo;
use luaur_vm::records::lua_debug::LuaDebug;
use luaur_vm::records::lua_state::lua_State;

pub unsafe extern "C" fn conformance_interrupt_inspection_hook(
    l: *mut lua_State,
    ar: *mut LuaDebug,
) {
    assert_ne!(0, lua_getinfo(l, 0, c"nsl".as_ptr(), ar));
}
