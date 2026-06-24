use alloc::string::String;
use core::ffi::{c_char, CStr};

use luaur_ast::records::parse_options::ParseOptions;
use luaur_bytecode::records::bytecode_encoder::BytecodeEncoder;
use luaur_compiler::functions::compile::compile;
use luaur_vm::functions::lua_checkstack::lua_checkstack;
use luaur_vm::functions::lua_debugtrace::lua_debugtrace;
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
use luaur_vm::macros::lua_getglobal::lua_getglobal;
use luaur_vm::macros::lua_isnil::lua_isnil;
use luaur_vm::macros::lua_minstack::LUA_MINSTACK;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::macros::lua_tostring::lua_tostring;
use luaur_vm::type_aliases::lua_state::lua_State;

use crate::functions::copts::copts;

pub unsafe fn run_code(l: *mut lua_State, source: &str) -> String {
    lua_checkstack(l, LUA_MINSTACK);

    struct NoopEncoder;
    impl BytecodeEncoder for NoopEncoder {
        fn encode(&mut self, _data: &mut [u32]) {}
    }
    let options = copts();
    let parse_options = ParseOptions::default();
    let mut encoder = NoopEncoder;
    let source_owned: String = source.into();
    let bytecode = compile(
        &source_owned,
        &options,
        &parse_options,
        &mut encoder as *mut dyn BytecodeEncoder,
    );

    if luau_load(
        l,
        c"=stdin".as_ptr(),
        bytecode.as_ptr() as *const c_char,
        bytecode.len(),
        0,
    ) != 0
    {
        let mut len: usize = 0;
        let msg = lua_tolstring(l, -1, &mut len as *mut usize);

        let error = lua_string_from(msg, len);
        lua_pop(l, 1);

        return error;
    }

    let t = lua_newthread(l);

    lua_pushvalue(l, -2);
    lua_remove(l, -3);
    lua_xmove(l, t, 1);

    let status = lua_resume(t, core::ptr::null_mut(), 0);

    if status == 0 {
        let n = lua_gettop(t);

        if n != 0 {
            lua_l_checkstack(t, LUA_MINSTACK, "too many results to print");
            lua_getglobal(t, c"_PRETTYPRINT".as_ptr());
            // If _PRETTYPRINT is nil, then use the standard print function instead
            if lua_isnil!(t, -1) {
                lua_pop(t, 1);
                lua_getglobal(t, c"print".as_ptr());
            }
            lua_insert(t, 1);
            lua_pcall(t, n, 0, 0);
        }

        lua_pop(l, 1);
        String::new()
    } else {
        let mut error: String;

        if status == luaur_vm::enums::lua_status::lua_Status::LUA_YIELD as i32 {
            error = "thread yielded unexpectedly".into();
        } else {
            let str_ptr = lua_tostring!(t, -1);
            if !str_ptr.is_null() {
                error = CStr::from_ptr(str_ptr).to_string_lossy().into_owned();
            } else {
                error = String::new();
            }
        }

        error.push_str("\nstack backtrace:\n");
        let trace = lua_debugtrace(t);
        if !trace.is_null() {
            error.push_str(&CStr::from_ptr(trace).to_string_lossy());
        }

        lua_pop(l, 1);
        error
    }
}

// Faithful port of `std::string error(msg, len)`: build a String from the raw
// (pointer, length) pair returned by lua_tolstring.
unsafe fn lua_string_from(msg: *const c_char, len: usize) -> String {
    if msg.is_null() {
        return String::new();
    }
    let bytes = core::slice::from_raw_parts(msg as *const u8, len);
    String::from_utf8_lossy(bytes).into_owned()
}
