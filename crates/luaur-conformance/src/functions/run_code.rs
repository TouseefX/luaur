use core::ffi::{c_char, c_int, c_void};

use luaur_compiler::functions::luau_compile::luau_compile;
use luaur_vm::functions::lua_pcall::lua_pcall;
use luaur_vm::functions::luau_load::luau_load;
use luaur_vm::macros::lua_multret::LUA_MULTRET;
use luaur_vm::records::lua_state::lua_State;

extern "C" {
    fn free(ptr: *mut c_void);
}

pub fn run_code(L: *mut lua_State, source: &str) -> c_int {
    unsafe {
        let mut bytecode_size = 0usize;
        let bytecode = luau_compile(
            source.as_ptr() as *const c_char,
            source.len(),
            core::ptr::null_mut(),
            &mut bytecode_size,
        );

        if luau_load(L, c"test".as_ptr(), bytecode, bytecode_size, 0) != 0 {
            free(bytecode as *mut c_void);
            return -1;
        }

        free(bytecode as *mut c_void);
        lua_pcall(L, 0, LUA_MULTRET, 0)
    }
}
