//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:3398:conformance_userdata_api`
//! Source: `tests/Conformance.test.cpp`

#[cfg(test)]
#[test]
fn conformance_userdata_api() {
    use core::ffi::c_void;
    use core::sync::atomic::Ordering;

    use crate::functions::userdata_api_dtor_hits::USERDATA_API_DTOR_HITS;
    use crate::functions::userdata_api_inline_char_dtor::userdata_api_inline_char_dtor;
    use crate::functions::userdata_api_inline_int_dtor::userdata_api_inline_int_dtor;
    use crate::functions::userdata_api_tag_dtor::userdata_api_tag_dtor;
    use crate::type_aliases::state_ref::StateRef;
    use luaur_vm::functions::lua_getuserdatadtor::lua_getuserdatadtor;
    use luaur_vm::functions::lua_getuserdatametatable::lua_getuserdatametatable;
    use luaur_vm::functions::lua_l_checkudata::lua_l_checkudata;
    use luaur_vm::functions::lua_l_newmetatable::lua_l_newmetatable;
    use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
    use luaur_vm::functions::lua_newuserdatadtor::lua_newuserdatadtor;
    use luaur_vm::functions::lua_newuserdatatagged::lua_newuserdatatagged;
    use luaur_vm::functions::lua_newuserdatataggedwithmetatable::lua_newuserdatataggedwithmetatable;
    use luaur_vm::functions::lua_pushlightuserdatatagged::lua_pushlightuserdatatagged;
    use luaur_vm::functions::lua_pushvalue::lua_pushvalue;
    use luaur_vm::functions::lua_setmetatable::lua_setmetatable;
    use luaur_vm::functions::lua_setuserdatadtor::lua_setuserdatadtor;
    use luaur_vm::functions::lua_setuserdatametatable::lua_setuserdatametatable;
    use luaur_vm::functions::lua_setuserdatatag::lua_setuserdatatag;
    use luaur_vm::functions::lua_tolightuserdata::lua_tolightuserdata;
    use luaur_vm::functions::lua_topointer::lua_topointer;
    use luaur_vm::functions::lua_touserdata::lua_touserdata;
    use luaur_vm::functions::lua_touserdatatagged::lua_touserdatatagged;
    use luaur_vm::functions::lua_userdatatag::lua_userdatatag;
    use luaur_vm::macros::lua_l_getmetatable::luaL_getmetatable;

    USERDATA_API_DTOR_HITS.store(0, Ordering::SeqCst);

    let global_state =
        StateRef::new(unsafe { lua_l_newstate() }).expect("lua state allocation failed");
    let l = global_state.as_ptr();

    unsafe {
        let dtor_is_null = lua_getuserdatadtor(l, 42).is_none();
        assert!(dtor_is_null);
        lua_setuserdatadtor(l, 42, Some(userdata_api_tag_dtor));
        let dtor_is_set = lua_getuserdatadtor(l, 42).map(|dtor| dtor as *const ())
            == Some(userdata_api_tag_dtor as *const ());
        assert!(dtor_is_set);

        let mut lud = 0i32;
        let lud_ptr = (&mut lud as *mut i32).cast::<c_void>();
        lua_pushlightuserdatatagged(l, lud_ptr, 0);

        assert_eq!(lua_tolightuserdata(l, -1), lud_ptr);
        assert_eq!(lua_touserdata(l, -1), lud_ptr);
        assert_eq!(lua_topointer(l, -1), lud_ptr.cast_const());

        let ud1 = lua_newuserdatatagged(l, 4, 0) as *mut i32;
        *ud1 = 42;

        assert!(lua_tolightuserdata(l, -1).is_null());
        assert_eq!(lua_touserdata(l, -1), ud1.cast::<c_void>());
        assert_eq!(lua_topointer(l, -1), ud1.cast::<c_void>().cast_const());

        let ud2 = lua_newuserdatatagged(l, 4, 42) as *mut i32;
        *ud2 = -4;

        assert_eq!(lua_touserdatatagged(l, -1, 42), ud2.cast::<c_void>());
        assert!(lua_touserdatatagged(l, -1, 41).is_null());
        assert_eq!(lua_userdatatag(l, -1), 42);

        lua_setuserdatatag(l, -1, 43);
        assert_eq!(lua_userdatatag(l, -1), 43);
        lua_setuserdatatag(l, -1, 42);

        let ud3 = lua_newuserdatadtor(l, 4, Some(userdata_api_inline_int_dtor)) as *mut i32;
        let ud4 = lua_newuserdatadtor(l, 1, Some(userdata_api_inline_char_dtor)) as *mut core::ffi::c_char;

        *ud3 = 43;
        *ud4 = 3;

        lua_l_newmetatable(l, c"udata1".as_ptr());
        lua_l_newmetatable(l, c"udata2".as_ptr());

        let ud5 = lua_newuserdatatagged(l, 0, 0);
        luaL_getmetatable(l, c"udata1".as_ptr());
        lua_setmetatable(l, -2);

        let ud6 = lua_newuserdatatagged(l, 0, 0);
        luaL_getmetatable(l, c"udata2".as_ptr());
        lua_setmetatable(l, -2);

        assert_eq!(lua_l_checkudata(l, -2, "udata1"), ud5);
        assert_eq!(lua_l_checkudata(l, -1, "udata2"), ud6);

        lua_l_newmetatable(l, c"udata3".as_ptr());
        lua_pushvalue(l, -1);
        lua_setuserdatametatable(l, 50);

        lua_l_newmetatable(l, c"udata4".as_ptr());
        lua_pushvalue(l, -1);
        lua_setuserdatametatable(l, 51);

        let ud7 = lua_newuserdatatagged(l, 16, 50);
        lua_getuserdatametatable(l, 50);
        lua_setmetatable(l, -2);

        let ud8 = lua_newuserdatataggedwithmetatable(l, 16, 51);

        assert_eq!(lua_l_checkudata(l, -2, "udata3"), ud7);
        assert_eq!(lua_l_checkudata(l, -1, "udata4"), ud8);
    }

    drop(global_state);

    assert_eq!(USERDATA_API_DTOR_HITS.load(Ordering::SeqCst), 42);
}
