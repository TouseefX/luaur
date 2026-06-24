//! `static std::string runCode(lua_State* L, const std::string& source)`
//! (`CLI/src/Web.cpp:71-140`).
//!
//! Compiles `source`, loads it into `L`, runs it on a fresh thread, prints any
//! results, and returns "" on success or a formatted error (with source:line
//! prefix and stack backtrace) on failure.

use alloc::string::String;
use alloc::string::ToString;
use core::ffi::{c_char, c_void};

use luaur_compiler::functions::luau_compile::luau_compile;
use luaur_vm::enums::lua_status::lua_Status;
use luaur_vm::functions::lua_debugtrace::lua_debugtrace;
use luaur_vm::functions::lua_getinfo::lua_getinfo;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_insert::lua_insert;
use luaur_vm::functions::lua_l_checkstack::lua_l_checkstack;
use luaur_vm::functions::lua_newthread::lua_newthread;
use luaur_vm::functions::lua_pcall::lua_pcall;
use luaur_vm::functions::lua_pushvalue::lua_pushvalue;
use luaur_vm::functions::lua_remove::lua_remove;
use luaur_vm::functions::lua_resume::lua_resume;
use luaur_vm::functions::lua_tolstring::lua_tolstring;
use luaur_vm::functions::lua_xmove::lua_xmove;
use luaur_vm::functions::luau_load::luau_load;
use luaur_vm::macros::lua_minstack::LUA_MINSTACK;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::records::lua_debug::LuaDebug;
use luaur_vm::type_aliases::lua_state::lua_State;

extern "C" {
    fn free(ptr: *mut c_void);
}

pub fn run_code(l: *mut lua_State, source: &str) -> String {
    unsafe {
        // size_t bytecodeSize = 0;
        // char* bytecode = luau_compile(source.data(), source.length(), nullptr, &bytecodeSize);
        let mut bytecode_size: usize = 0;
        let bytecode = luau_compile(
            source.as_ptr() as *const c_char,
            source.len(),
            core::ptr::null_mut(),
            &mut bytecode_size,
        );

        // int result = luau_load(L, "=stdin", bytecode, bytecodeSize, 0);
        let result = luau_load(l, c"=stdin".as_ptr(), bytecode, bytecode_size, 0);

        // free(bytecode);
        free(bytecode as *mut c_void);

        if result != 0 {
            // const char* msg = lua_tolstring(L, -1, &len);
            let mut len: usize = 0;
            let msg = lua_tolstring(l, -1, &mut len);

            let error =
                core::str::from_utf8_unchecked(core::slice::from_raw_parts(msg as *const u8, len))
                    .to_string();
            lua_pop(l, 1);

            return error;
        }

        // lua_State* T = lua_newthread(L);
        let t = lua_newthread(l);

        // lua_pushvalue(L, -2);
        // lua_remove(L, -3);
        // lua_xmove(L, T, 1);
        lua_pushvalue(l, -2);
        lua_remove(l, -3);
        lua_xmove(l, t, 1);

        // int status = lua_resume(T, NULL, 0);
        let status = lua_resume(t, core::ptr::null_mut(), 0);

        if status == 0 {
            let n = lua_gettop(t);

            if n != 0 {
                lua_l_checkstack(t, LUA_MINSTACK, "too many results to print");
                luaur_vm::macros::lua_getglobal::lua_getglobal(t, c"print".as_ptr());
                lua_insert(t, 1);
                lua_pcall(t, n, 0, 0);
            }

            lua_pop(l, 1); // pop T
            String::new()
        } else {
            let mut error = String::new();

            // lua_Debug ar;
            // if (lua_getinfo(L, 0, "sln", &ar))
            let mut ar = LuaDebug {
                name: core::ptr::null(),
                what: core::ptr::null(),
                source: core::ptr::null(),
                short_src: core::ptr::null(),
                linedefined: 0,
                currentline: 0,
                nupvals: 0,
                nparams: 0,
                isvararg: 0,
                userdata: core::ptr::null_mut(),
                ssbuf: [0; 256],
            };
            if lua_getinfo(l, 0, c"sln".as_ptr(), &mut ar) != 0 {
                if !ar.short_src.is_null() {
                    error.push_str(
                        &core::ffi::CStr::from_ptr(ar.short_src).to_string_lossy(),
                    );
                }
                error.push(':');
                error.push_str(&ar.currentline.to_string());
                error.push_str(": ");
            }

            if status == lua_Status::LUA_YIELD as i32 {
                error.push_str("thread yielded unexpectedly");
            } else {
                // else if (const char* str = lua_tostring(T, -1))
                let str_ptr = lua_tolstring(t, -1, core::ptr::null_mut());
                if !str_ptr.is_null() {
                    error.push_str(&core::ffi::CStr::from_ptr(str_ptr).to_string_lossy());
                }
            }

            error.push_str("\nstack backtrace:\n");
            let trace = lua_debugtrace(t);
            if !trace.is_null() {
                error.push_str(&core::ffi::CStr::from_ptr(trace).to_string_lossy());
            }

            lua_pop(l, 1); // pop T
            error
        }
    }
}
