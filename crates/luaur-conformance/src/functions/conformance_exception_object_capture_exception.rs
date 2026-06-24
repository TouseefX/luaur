use alloc::string::String;
use core::ffi::{c_char, CStr};
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

use crate::records::exception_result::ExceptionResult;
use luaur_vm::functions::lua_call::lua_call;
use luaur_vm::functions::lua_getfield::lua_getfield;
use luaur_vm::functions::lua_is_lfunction::lua_is_lfunction;
use luaur_vm::functions::lua_newthread::lua_newthread;
use luaur_vm::macros::lua_globalsindex::LUA_GLOBALSINDEX;
use luaur_vm::records::lua_exception::lua_exception;
use luaur_vm::records::lua_state::lua_State;

pub unsafe fn conformance_exception_object_capture_exception(
    l: *mut lua_State,
    function_to_run: *const c_char,
) -> ExceptionResult {
    let result = catch_unwind(AssertUnwindSafe(|| {
        let thread_state = lua_newthread(l);
        lua_getfield(thread_state, LUA_GLOBALSINDEX, function_to_run);
        assert_ne!(lua_is_lfunction(thread_state, -1), 0);
        lua_call(thread_state, 0, 0);
    }));

    match result {
        Ok(()) => ExceptionResult {
            exception_generated: false,
            description: String::new(),
        },
        Err(payload) => {
            if let Some(e) = payload.downcast_ref::<lua_exception>() {
                let what = e.what();
                assert!(!what.is_null());
                ExceptionResult {
                    exception_generated: true,
                    description: CStr::from_ptr(what).to_string_lossy().into_owned(),
                }
            } else {
                resume_unwind(payload);
            }
        }
    }
}
