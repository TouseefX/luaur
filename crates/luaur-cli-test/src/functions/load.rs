use core::ffi::{c_char, c_int, c_void, CStr};

use luaur_cli_lib::functions::read_file::read_file;
use luaur_compiler::functions::luau_compile::luau_compile;
use luaur_vm::enums::lua_status::lua_Status;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_isstring::lua_isstring;
use luaur_vm::functions::lua_l_sandboxthread::lua_l_sandboxthread;
use luaur_vm::functions::lua_mainthread::lua_mainthread;
use luaur_vm::functions::lua_newthread::lua_newthread;
use luaur_vm::functions::lua_remove::lua_remove;
use luaur_vm::functions::lua_resume::lua_resume;
use luaur_vm::functions::lua_xmove::lua_xmove;
use luaur_vm::functions::luau_load::luau_load;
use luaur_vm::macros::lua_l_error::luaL_error;
use luaur_vm::macros::lua_tostring::lua_tostring;
use luaur_vm::records::lua_state::lua_State;

extern "C" {
    fn free(ptr: *mut c_void);
}

#[allow(non_snake_case)]
pub unsafe extern "C-unwind" fn load(
    l: *mut c_void,
    _ctx: *mut c_void,
    _path: *const c_char,
    chunkname: *const c_char,
    loadname: *const c_char,
) -> c_int {
    let l = l as *mut lua_State;

    // module needs to run in a new thread, isolated from the rest
    // note: we create ML on main thread so that it doesn't inherit environment of L
    let gl = lua_mainthread(l);
    let ml = lua_newthread(gl);
    lua_xmove(gl, l, 1);

    // new thread needs to have the globals sandboxed
    lua_l_sandboxthread(ml);

    let mut had_contents = false;
    let mut status: c_int = lua_Status::LUA_OK as c_int;

    // Handle owned buffers in a scope which doesn't cause a Luau error
    {
        let contents = read_file(&CStr::from_ptr(loadname).to_string_lossy());
        had_contents = contents.is_some();

        if let Some(contents) = contents {
            // now we can compile & run module on the new thread
            let mut bytecode_size = 0usize;
            let bytecode = luau_compile(
                contents.as_ptr() as *const c_char,
                contents.len(),
                core::ptr::null_mut(),
                &mut bytecode_size,
            );

            status = luau_load(ml, chunkname, bytecode, bytecode_size, 0);

            free(bytecode as *mut c_void);
        }
    }

    if !had_contents {
        let loadname_str = CStr::from_ptr(loadname).to_string_lossy();
        luaL_error!(l, "could not read file '{}'", loadname_str);
    }

    if status == 0 {
        // codegen / coverage / counters are inactive in this context

        let status = lua_resume(ml, l, 0);

        if status == 0 {
            if lua_gettop(ml) != 1 {
                luaL_error!(l, "module must return a single value");
            }
        } else if status == lua_Status::LUA_YIELD as c_int {
            luaL_error!(l, "module can not yield");
        } else if lua_isstring(ml, -1) == 0 {
            luaL_error!(l, "unknown error while running module");
        } else {
            let msg = lua_tostring!(ml, -1);
            let msg_str = if msg.is_null() {
                alloc::borrow::Cow::Borrowed("")
            } else {
                CStr::from_ptr(msg).to_string_lossy()
            };
            luaL_error!(l, "error while running module: {}", msg_str);
        }
    }

    // add ML result to L stack
    lua_xmove(ml, l, 1);

    // remove ML thread from L stack
    lua_remove(l, -2);

    // added one value to L stack: module result
    1
}
