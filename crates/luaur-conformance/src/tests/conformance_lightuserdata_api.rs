//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:3546:conformance_lightuserdata_api`
//! Source: `tests/Conformance.test.cpp`

#[cfg(test)]
#[test]
fn conformance_lightuserdata_api() {
    use core::ffi::{c_void, CStr};

    use crate::type_aliases::state_ref::StateRef;
    use luaur_vm::functions::lua_createtable::lua_createtable;
    use luaur_vm::functions::lua_getlightuserdataname::lua_getlightuserdataname;
    use luaur_vm::functions::lua_gettable::lua_gettable;
    use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
    use luaur_vm::functions::lua_l_typename::lua_l_typename;
    use luaur_vm::functions::lua_lightuserdatatag::lua_lightuserdatatag;
    use luaur_vm::functions::lua_pushinteger::lua_pushinteger;
    use luaur_vm::functions::lua_pushlightuserdatatagged::lua_pushlightuserdatatagged;
    use luaur_vm::functions::lua_pushstring::lua_pushstring;
    use luaur_vm::functions::lua_rawequal::lua_rawequal;
    use luaur_vm::functions::lua_setfield::lua_setfield;
    use luaur_vm::functions::lua_setlightuserdataname::lua_setlightuserdataname;
    use luaur_vm::functions::lua_setmetatable::lua_setmetatable;
    use luaur_vm::functions::lua_settable::lua_settable;
    use luaur_vm::functions::lua_tolightuserdatatagged::lua_tolightuserdatatagged;
    use luaur_vm::macros::lua_pop::lua_pop;

    let global_state =
        StateRef::new(unsafe { lua_l_newstate() }).expect("lua state allocation failed");
    let l = global_state.as_ptr();

    let value = 0x12345678usize as *mut c_void;

    unsafe {
        lua_pushlightuserdatatagged(l, value, 1);
        assert_eq!(lua_lightuserdatatag(l, -1), 1);
        assert!(lua_tolightuserdatatagged(l, -1, 0).is_null());
        assert_eq!(lua_tolightuserdatatagged(l, -1, 1), value);

        lua_setlightuserdataname(l, 1, c"id".as_ptr());
        assert!(lua_getlightuserdataname(l, 0).is_null());
        assert_eq!(CStr::from_ptr(lua_getlightuserdataname(l, 1)), c"id");
        assert_eq!(CStr::from_ptr(lua_l_typename(l, -1)), c"id");
        lua_pop(l, 1);

        lua_pushlightuserdatatagged(l, value, 0);
        lua_pushlightuserdatatagged(l, value, 1);
        assert_eq!(lua_rawequal(l, -1, -2), 0);
        lua_pop(l, 2);

        lua_createtable(l, 0, 0);

        lua_pushlightuserdatatagged(l, value, 2);
        lua_pushinteger(l, 20);
        lua_settable(l, -3);
        lua_pushlightuserdatatagged(l, value, 3);
        lua_pushinteger(l, 30);
        lua_settable(l, -3);

        lua_pushlightuserdatatagged(l, value, 2);
        lua_gettable(l, -2);
        lua_pushinteger(l, 20);
        assert_eq!(lua_rawequal(l, -1, -2), 1);
        lua_pop(l, 2);

        lua_pushlightuserdatatagged(l, value, 3);
        lua_gettable(l, -2);
        lua_pushinteger(l, 30);
        assert_eq!(lua_rawequal(l, -1, -2), 1);
        lua_pop(l, 2);

        lua_pop(l, 1);

        lua_pushlightuserdatatagged(l, value, 0);
        assert_eq!(CStr::from_ptr(lua_l_typename(l, -1)), c"userdata");

        lua_createtable(l, 0, 1);
        lua_pushstring(l, c"luserdata".as_ptr());
        lua_setfield(l, -2, c"__type".as_ptr());
        assert_eq!(lua_setmetatable(l, -2), 1);

        assert_eq!(CStr::from_ptr(lua_l_typename(l, -1)), c"luserdata");
        lua_pop(l, 1);
    }
}
