use alloc::string::String;
use core::ffi::{c_char, c_void};

use luaur_compiler::functions::luau_compile::luau_compile;
use luaur_vm::functions::lua_checkstack::lua_checkstack;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_insert::lua_insert;
use luaur_vm::functions::lua_l_checkstack::lua_l_checkstack;
use luaur_vm::functions::lua_newthread::lua_newthread;
use luaur_vm::functions::lua_pcall::lua_pcall;
use luaur_vm::functions::lua_pushvalue::lua_pushvalue;
use luaur_vm::functions::lua_remove::lua_remove;
use luaur_vm::functions::lua_resume::lua_resume;
use luaur_vm::functions::lua_tolstring::lua_tolstring;
use luaur_vm::functions::lua_type::lua_type;
use luaur_vm::functions::lua_xmove::lua_xmove;
use luaur_vm::functions::luau_load::luau_load;
use luaur_vm::macros::lua_minstack::LUA_MINSTACK;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::records::lua_state::lua_State;

pub fn run_code(l: *mut lua_State, source: &String) -> String {
    unsafe {
        lua_checkstack(l, LUA_MINSTACK);

        let mut bytecode_size = 0usize;
        let bytecode = luau_compile(
            source.as_ptr() as *const c_char,
            source.len(),
            core::ptr::null_mut(),
            &mut bytecode_size,
        );

        let status = luau_load(l, c"=stdin".as_ptr(), bytecode, bytecode_size, 0);

        free(bytecode as *mut c_void);

        if status != 0 {
            let mut len = 0usize;
            let msg = lua_tolstring(l, -1, &mut len);
            let error =
                core::str::from_utf8_unchecked(core::slice::from_raw_parts(msg as *const u8, len))
                    .to_string();
            lua_pop(l, 1);
            return error;
        }

        let thread = lua_newthread(l);

        lua_pushvalue(l, -2);
        lua_remove(l, -3);
        lua_xmove(l, thread, 1);

        let status = lua_resume(thread, core::ptr::null_mut(), 0);

        if status == 0 {
            let n = lua_gettop(thread);

            if n != 0 {
                lua_l_checkstack(thread, LUA_MINSTACK, "too many results to print");
                luaur_vm::macros::lua_getglobal::lua_getglobal(thread, c"_PRETTYPRINT".as_ptr());

                if lua_type(thread, -1) == luaur_vm::enums::lua_type::lua_Type::LUA_TNIL as i32 {
                    lua_pop(thread, 1);
                    luaur_vm::macros::lua_getglobal::lua_getglobal(thread, c"print".as_ptr());
                }

                lua_insert(thread, 1);
                lua_pcall(thread, n, 0, 0);
            }

            lua_pop(l, 1);
            String::new()
        } else {
            let mut len = 0usize;
            let msg = lua_tolstring(thread, -1, &mut len);
            let error =
                core::str::from_utf8_unchecked(core::slice::from_raw_parts(msg as *const u8, len))
                    .to_string();

            lua_pop(l, 1);
            error
        }
    }
}

extern "C" {
    fn free(ptr: *mut c_void);
}
