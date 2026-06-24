//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:4202:conformance_huge_function_load_failure`
//! Source: `tests/Conformance.test.cpp`

#[cfg(test)]
#[test]
fn conformance_huge_function_load_failure() {
    use core::ffi::{c_char, c_void, CStr};
    use core::sync::atomic::Ordering;

    use crate::functions::huge_function_load_failure_test_allocate::{
        huge_function_load_failure_test_allocate,
        HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_COUNT,
        HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_TO_FAIL,
    };
    use crate::functions::make_huge_function_source::make_huge_function_source;
    use crate::type_aliases::state_ref::StateRef;
    use luaur_compiler::functions::luau_compile::luau_compile;
    use luaur_vm::functions::lua_c_fullgc::luaC_fullgc;
    use luaur_vm::functions::lua_l_openlibs::lua_l_openlibs;
    use luaur_vm::functions::lua_l_sandbox::lua_l_sandbox;
    use luaur_vm::functions::lua_l_sandboxthread::lua_l_sandboxthread;
    use luaur_vm::functions::lua_newstate::lua_newstate;
    use luaur_vm::functions::luau_load::luau_load;
    use luaur_vm::macros::lua_tostring::lua_tostring;

    extern "C" {
        fn free(ptr: *mut c_void);
    }

    let source = make_huge_function_source();
    let expected_total_large_allocations = 2usize;

    unsafe {
        let mut bytecode_size = 0usize;
        let bytecode = luau_compile(
            source.as_ptr() as *const c_char,
            source.len(),
            core::ptr::null_mut(),
            &mut bytecode_size,
        );
        assert!(!bytecode.is_null());

        let mut large_allocation_to_fail = 0usize;
        while large_allocation_to_fail != expected_total_large_allocations {
            HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_TO_FAIL
                .store(large_allocation_to_fail, Ordering::SeqCst);
            HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_COUNT.store(0, Ordering::SeqCst);

            let global_state = StateRef::new(lua_newstate(
                Some(huge_function_load_failure_test_allocate),
                core::ptr::null_mut(),
            ))
            .expect("lua state allocation failed");
            let l = global_state.as_ptr();

            lua_l_openlibs(l);
            lua_l_sandbox(l);
            lua_l_sandboxthread(l);

            let status = luau_load(l, c"=HugeFunction".as_ptr(), bytecode, bytecode_size, 0);
            assert_eq!(status, 1);

            assert_eq!(CStr::from_ptr(lua_tostring!(l, -1)), c"not enough memory");

            luaC_fullgc(l);

            large_allocation_to_fail += 1;
        }

        free(bytecode as *mut c_void);

        assert_eq!(large_allocation_to_fail, expected_total_large_allocations);
    }
}
