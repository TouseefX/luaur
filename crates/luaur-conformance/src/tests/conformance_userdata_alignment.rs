//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:3507:conformance_userdata_alignment`
//! Source: `tests/Conformance.test.cpp`

#[cfg(test)]
#[test]
fn conformance_userdata_alignment() {
    use crate::functions::userdata_alignment_alloc::userdata_alignment_alloc;
    use crate::functions::userdata_alignment_dtor::userdata_alignment_dtor;
    use crate::type_aliases::state_ref::StateRef;
    use luaur_vm::functions::lua_newstate::lua_newstate;
    use luaur_vm::functions::lua_newuserdatadtor::lua_newuserdatadtor;
    use luaur_vm::macros::lua_newuserdata::lua_newuserdata;
    use luaur_vm::macros::lua_pop::lua_pop;

    let global_state = StateRef::new(unsafe {
        lua_newstate(Some(userdata_alignment_alloc), core::ptr::null_mut())
    })
    .expect("lua state allocation failed");
    let l = global_state.as_ptr();

    unsafe {
        for size in (16..=4096).step_by(4) {
            for _ in 0..10 {
                let data = lua_newuserdata(l, size);
                assert_eq!((data as usize) % 16, 0);
                lua_pop(l, 1);
            }

            for _ in 0..10 {
                let data = lua_newuserdatadtor(l, size, Some(userdata_alignment_dtor));
                assert_eq!((data as usize) % 16, 0);
                lua_pop(l, 1);
            }
        }
    }
}
