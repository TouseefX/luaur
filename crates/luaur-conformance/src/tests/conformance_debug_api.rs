//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:3608:conformance_debug_api`
//! Source: `tests/Conformance.test.cpp`

#[cfg(test)]
#[test]
fn conformance_debug_api() {
    use crate::type_aliases::state_ref::StateRef;
    use luaur_vm::functions::lua_getinfo::lua_getinfo;
    use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
    use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
    use luaur_vm::records::lua_debug::LuaDebug;

    let global_state =
        StateRef::new(unsafe { lua_l_newstate() }).expect("lua state allocation failed");
    let l = global_state.as_ptr();

    unsafe {
        lua_pushnumber(l, 10.0);

        let mut ar: LuaDebug = core::mem::zeroed();
        assert_eq!(lua_getinfo(l, -1, c"f".as_ptr(), &mut ar), 0);
        assert_eq!(lua_getinfo(l, -10, c"f".as_ptr(), &mut ar), 0);
    }
}
