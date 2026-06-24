use core::ffi::c_int;

use luaur_compiler::functions::luau_compile::luau_compile;
use luaur_vm::functions::lua_insert::lua_insert;
use luaur_vm::functions::lua_l_checklstring::lua_l_checklstring;
use luaur_vm::functions::lua_pushnil::lua_pushnil;
use luaur_vm::functions::lua_setsafeenv::lua_setsafeenv;
use luaur_vm::functions::luau_load::luau_load;
use luaur_vm::luaL_optstring;
use luaur_vm::macros::lua_environindex::LUA_ENVIRONINDEX;
use luaur_vm::records::lua_state::lua_State;

extern "C" {
    fn free(ptr: *mut core::ffi::c_void);
}

pub unsafe extern "C" fn lua_loadstring(L: *mut lua_State) -> c_int {
    let mut len = 0usize;
    let source = lua_l_checklstring(L, 1, &mut len);
    let chunkname = luaL_optstring!(L, 2, source);

    lua_setsafeenv(L, LUA_ENVIRONINDEX, 0);

    let mut bytecode_size = 0usize;
    let bytecode = luau_compile(source, len, core::ptr::null_mut(), &mut bytecode_size);
    let result = luau_load(L, chunkname, bytecode, bytecode_size, 0);
    free(bytecode as *mut core::ffi::c_void);

    if result == 0 {
        return 1;
    }

    lua_pushnil(L);
    lua_insert(L, -2);
    2
}
