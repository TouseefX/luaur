//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:2502:conformance_api_calls`
//! Source: `tests/Conformance.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Conformance.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file VM/include/lua.h
//!   - includes -> source_file VM/include/lualib.h
//!   - includes -> source_file Compiler/include/luacode.h
//!   - includes -> source_file CodeGen/include/luacodegen.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Compiler/include/Luau/Compiler.h
//!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
//!   - includes -> source_file CodeGen/include/Luau/BytecodeSummary.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file tests/ConformanceIrHooks.h
//! - incoming:
//!   - declares <- source_file tests/Conformance.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias StateRef (tests/Conformance.test.cpp)
//!   - calls -> function runConformance (tests/Conformance.test.cpp)
//!   - calls -> function lua_newstate (VM/src/lstate.cpp)
//!   - calls -> function limitedRealloc (tests/Conformance.test.cpp)
//!   - calls -> function lua_call (VM/src/lapi.cpp)
//!   - calls -> function lua_getfield (VM/src/lapi.cpp)
//!   - calls -> function lua_pushnumber (VM/src/lapi.cpp)
//!   - calls -> function lua_isnumber (VM/src/lapi.cpp)
//!   - calls -> macro lua_tonumber (VM/include/lua.h)
//!   - calls -> macro lua_pop (VM/include/lua.h)
//!   - calls -> function lua_pushinteger (VM/src/lapi.cpp)
//!   - calls -> function lua_gettop (VM/src/lapi.cpp)
//!   - calls -> function lua_pcall (VM/src/lapi.cpp)
//!   - calls -> function lua_toboolean (VM/src/lapi.cpp)
//!   - calls -> function lua_cpcall (VM/src/lapi.cpp)
//!   - calls -> function cpcallTest (tests/Conformance.test.cpp)
//!   - calls -> function lua_status (VM/src/lapi.cpp)
//!   - calls -> macro lua_getglobal (VM/include/lua.h)
//!   - calls -> function luaL_checkinteger (VM/src/laux.cpp)
//!   - calls -> function lua_isstring (VM/src/lapi.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> macro lua_tostring (VM/include/lua.h)
//!   - calls -> function luaL_checkstack (VM/src/laux.cpp)
//!   - calls -> function lua_resume (VM/src/ldo.cpp)
//!   - calls -> function lua_newthread (VM/src/lapi.cpp)
//!   - calls -> function lua_pushcclosurek (VM/src/lapi.cpp)
//!   - calls -> function lua_isyieldable (VM/src/ldo.cpp)
//!   - calls -> function lua_equal (VM/src/lapi.cpp)
//!   - calls -> function lua_gc (VM/src/lapi.cpp)
//!   - calls -> function lua_clonefunction (VM/src/lapi.cpp)
//!   - calls -> macro lua_newtable (VM/include/lua.h)
//!   - calls -> function lua_setfield (VM/src/lapi.cpp)
//!   - calls -> function lua_setfenv (VM/src/lapi.cpp)
//!   - translates_to -> rust_item conformance_api_calls

#[cfg(test)]
#[test]
fn conformance_api_calls() {
    use crate::functions::conformance_api_calls_check_not_yieldable::conformance_api_calls_check_not_yieldable;
    use crate::functions::cpcall_test::cpcallTest;
    use crate::functions::limited_realloc::limited_realloc;
    use crate::functions::run_conformance::runConformance;
    use core::ffi::{c_char, c_void, CStr};
    use luaur_vm::enums::lua_gc_op::lua_GCOp;
    use luaur_vm::enums::lua_status::lua_Status;
    use luaur_vm::functions::lua_call::lua_call;
    use luaur_vm::functions::lua_clonefunction::lua_clonefunction;
    use luaur_vm::functions::lua_cpcall::lua_cpcall;
    use luaur_vm::functions::lua_equal::lua_equal;
    use luaur_vm::functions::lua_gc::lua_gc;
    use luaur_vm::functions::lua_getfield::lua_getfield;
    use luaur_vm::functions::lua_gettop::lua_gettop;
    use luaur_vm::functions::lua_isnumber::lua_isnumber;
    use luaur_vm::functions::lua_isstring::lua_isstring;
    use luaur_vm::functions::lua_l_checkinteger::lua_l_checkinteger;
    use luaur_vm::functions::lua_l_checkstack::lua_l_checkstack;
    use luaur_vm::functions::lua_newstate::lua_newstate;
    use luaur_vm::functions::lua_newthread::lua_newthread;
    use luaur_vm::functions::lua_pcall::lua_pcall;
    use luaur_vm::functions::lua_pushcclosurek::lua_pushcclosurek;
    use luaur_vm::functions::lua_pushinteger::lua_pushinteger;
    use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
    use luaur_vm::functions::lua_resume::lua_resume;
    use luaur_vm::functions::lua_setfenv::lua_setfenv;
    use luaur_vm::functions::lua_setfield::lua_setfield;
    use luaur_vm::functions::lua_status::lua_status;
    use luaur_vm::functions::lua_toboolean::lua_toboolean;
    use luaur_vm::macros::lua_globalsindex::LUA_GLOBALSINDEX;
    use luaur_vm::macros::lua_multret::LUA_MULTRET;
    use luaur_vm::macros::lua_newtable::lua_newtable;
    use luaur_vm::macros::lua_pop::lua_pop;
    use luaur_vm::macros::lua_tonumber::lua_tonumber;
    use luaur_vm::macros::lua_tostring::lua_tostring;
    use luaur_vm::macros::luai_maxcstack::LUAI_MAXCSTACK;

    let global_state = runConformance(
        c"apicalls.luau".as_ptr(),
        None,
        None,
        unsafe { lua_newstate(Some(limited_realloc), core::ptr::null_mut()) },
        core::ptr::null_mut(),
        false,
        core::ptr::null_mut(),
    );
    let l = global_state.as_ptr();

    unsafe {
        lua_getfield(l, LUA_GLOBALSINDEX, c"add".as_ptr());
        lua_pushnumber(l, 40.0);
        lua_pushnumber(l, 2.0);
        lua_call(l, 2, 1);
        assert_ne!(lua_isnumber(l, -1), 0);
        assert_eq!(lua_tonumber!(l, -1), 42.0);
        lua_pop(l, 1);

        lua_getfield(l, LUA_GLOBALSINDEX, c"getnresults".as_ptr());
        lua_pushinteger(l, 200);
        lua_call(l, 1, LUA_MULTRET);
        assert_eq!(lua_gettop(l), 200);
        lua_pop(l, 200);

        lua_getfield(l, LUA_GLOBALSINDEX, c"add".as_ptr());
        lua_pushnumber(l, 40.0);
        lua_pushnumber(l, 2.0);
        let status = lua_pcall(l, 2, 1, 0);
        assert_eq!(status, lua_Status::LUA_OK as i32);
        assert_ne!(lua_isnumber(l, -1), 0);
        assert_eq!(lua_tonumber!(l, -1), 42.0);
        lua_pop(l, 1);

        lua_getfield(l, LUA_GLOBALSINDEX, c"getnresults".as_ptr());
        lua_pushinteger(l, 200);
        let status = lua_pcall(l, 1, LUA_MULTRET, 0);
        assert_eq!(status, lua_Status::LUA_OK as i32);
        assert_eq!(lua_gettop(l), 200);
        lua_pop(l, 200);

        lua_getfield(l, LUA_GLOBALSINDEX, c"pcall".as_ptr());
        lua_getfield(l, LUA_GLOBALSINDEX, c"getnresults".as_ptr());
        lua_pushinteger(l, 200);
        lua_call(l, 2, LUA_MULTRET);
        assert_eq!(lua_gettop(l), 201);
        lua_pop(l, 200);
        assert_eq!(lua_toboolean(l, -1), 1);
        lua_pop(l, 1);

        let mut should_fail = false;
        assert_eq!(
            lua_cpcall(
                l,
                Some(cpcallTest),
                (&mut should_fail as *mut bool).cast::<c_void>(),
            ),
            lua_Status::LUA_OK as i32
        );
        assert_eq!(lua_status(l), lua_Status::LUA_OK as i32);

        lua_getfield(l, LUA_GLOBALSINDEX, c"cpcallvalue".as_ptr());
        assert_eq!(lua_l_checkinteger(l, -1), 123);
        lua_pop(l, 1);

        let mut should_fail = true;
        assert_eq!(
            lua_cpcall(
                l,
                Some(cpcallTest),
                (&mut should_fail as *mut bool).cast::<c_void>(),
            ),
            lua_Status::LUA_ERRRUN as i32
        );
        assert_ne!(lua_isstring(l, -1), 0);
        assert_eq!(CStr::from_ptr(lua_tostring!(l, -1)), c"Failed");
        lua_pop(l, 1);

        assert_eq!(lua_status(l), lua_Status::LUA_OK as i32);

        let mut should_fail = false;
        assert_eq!(lua_gettop(l), 0);
        lua_l_checkstack(l, LUAI_MAXCSTACK - 1, "must succeed");

        for _ in 0..LUAI_MAXCSTACK - 1 {
            lua_pushnumber(l, 1.0);
        }

        assert_eq!(
            lua_cpcall(
                l,
                Some(cpcallTest),
                (&mut should_fail as *mut bool).cast::<c_void>(),
            ),
            lua_Status::LUA_ERRRUN as i32
        );
        assert_ne!(lua_isstring(l, -1), 0);
        assert_eq!(CStr::from_ptr(lua_tostring!(l, -1)), c"stack limit");
        lua_pop(l, 1);

        assert_eq!(lua_status(l), lua_Status::LUA_OK as i32);
        lua_pop(l, LUAI_MAXCSTACK - 1);

        let l2 = lua_newthread(l);
        lua_pushcclosurek(
            l2,
            Some(conformance_api_calls_check_not_yieldable),
            core::ptr::null(),
            0,
            None,
        );
        lua_call(l2, 0, 0);

        lua_getfield(l2, LUA_GLOBALSINDEX, c"getnresults".as_ptr());
        lua_pushinteger(l2, 1);
        let status = lua_resume(l2, core::ptr::null_mut(), 1);
        assert_eq!(status, lua_Status::LUA_OK as i32);
        assert_eq!(lua_gettop(l2), 1);
        lua_pop(l2, 1);

        lua_pushcclosurek(
            l2,
            Some(conformance_api_calls_check_not_yieldable),
            core::ptr::null(),
            0,
            None,
        );
        lua_call(l2, 0, 0);

        lua_pop(l, 1);

        let l2 = lua_newthread(l);

        lua_getfield(l2, LUA_GLOBALSINDEX, c"create_with_tm".as_ptr());
        lua_pushnumber(l2, 42.0);
        lua_pcall(l2, 1, 1, 0);

        lua_getfield(l2, LUA_GLOBALSINDEX, c"create_with_tm".as_ptr());
        lua_pushnumber(l2, 42.0);
        lua_pcall(l2, 1, 1, 0);

        lua_gc(l2, lua_GCOp::LUA_GCCOLLECT as i32, 0);
        lua_gc(l2, lua_GCOp::LUA_GCSTEP as i32, 8);

        assert_eq!(lua_equal(l2, -1, -2), 1);
        lua_pop(l2, 2);

        lua_pop(l, 1);

        lua_getfield(l, LUA_GLOBALSINDEX, c"getpi".as_ptr());
        lua_call(l, 0, 1);
        assert_eq!(lua_tonumber!(l, -1), 3.1415926);
        lua_pop(l, 1);

        lua_getfield(l, LUA_GLOBALSINDEX, c"getpi".as_ptr());

        lua_clonefunction(l, -1);
        lua_newtable(l);
        lua_pushnumber(l, 42.0);
        lua_setfield(l, -2, c"pi".as_ptr());
        lua_setfenv(l, -2);

        lua_call(l, 0, 1);
        assert_eq!(lua_tonumber!(l, -1), 42.0);
        lua_pop(l, 1);

        lua_call(l, 0, 1);
        assert_eq!(lua_tonumber!(l, -1), 3.1415926);
        lua_pop(l, 1);

        lua_getfield(l, LUA_GLOBALSINDEX, c"incuv".as_ptr());
        lua_call(l, 0, 1);
        assert_eq!(lua_tonumber!(l, -1), 1.0);
        lua_pop(l, 1);

        lua_getfield(l, LUA_GLOBALSINDEX, c"incuv".as_ptr());
        lua_clonefunction(l, -1);
        lua_clonefunction(l, -2);

        lua_call(l, 0, 1);
        assert_eq!(lua_tonumber!(l, -1), 2.0);
        lua_pop(l, 1);

        lua_call(l, 0, 1);
        assert_eq!(lua_tonumber!(l, -1), 3.0);
        lua_pop(l, 1);

        lua_call(l, 0, 1);
        assert_eq!(lua_tonumber!(l, -1), 4.0);
        lua_pop(l, 1);

        lua_getfield(l, LUA_GLOBALSINDEX, c"largealloc".as_ptr());
        let res = lua_pcall(l, 0, 0, 0);
        assert_eq!(res, lua_Status::LUA_ERRMEM as i32);
        lua_pop(l, 1);

        lua_getfield(l, LUA_GLOBALSINDEX, c"oops".as_ptr());
        lua_getfield(l, LUA_GLOBALSINDEX, c"largealloc".as_ptr());
        let res = lua_pcall(l, 0, 1, -2);
        assert_eq!(res, lua_Status::LUA_ERRMEM as i32);
        assert_ne!(lua_isstring(l, -1), 0);
        assert_eq!(CStr::from_ptr(lua_tostring!(l, -1)), c"oops");
        lua_pop(l, 2);

        lua_getfield(l, LUA_GLOBALSINDEX, c"error".as_ptr());
        lua_getfield(l, LUA_GLOBALSINDEX, c"largealloc".as_ptr());
        let res = lua_pcall(l, 0, 1, -2);
        assert_eq!(res, lua_Status::LUA_ERRERR as i32);
        assert_ne!(lua_isstring(l, -1), 0);
        assert_eq!(
            CStr::from_ptr(lua_tostring!(l, -1)),
            c"error in error handling"
        );
        lua_pop(l, 2);

        lua_getfield(l, LUA_GLOBALSINDEX, c"largealloc".as_ptr());
        lua_getfield(l, LUA_GLOBALSINDEX, c"largealloc".as_ptr());
        let res = lua_pcall(l, 0, 1, -2);
        assert_eq!(res, lua_Status::LUA_ERRMEM as i32);
        assert_ne!(lua_isstring(l, -1), 0);
        assert_eq!(CStr::from_ptr(lua_tostring!(l, -1)), c"not enough memory");
        lua_pop(l, 2);

        lua_getfield(l, LUA_GLOBALSINDEX, c"largealloc".as_ptr());
        lua_getfield(l, LUA_GLOBALSINDEX, c"error".as_ptr());
        let res = lua_pcall(l, 0, 1, -2);
        assert_eq!(res, lua_Status::LUA_ERRERR as i32);
        assert_ne!(lua_isstring(l, -1), 0);
        assert_eq!(
            CStr::from_ptr(lua_tostring!(l, -1)),
            c"error in error handling"
        );
        lua_pop(l, 2);

        assert_eq!(lua_gettop(l), 0);
    }
}
