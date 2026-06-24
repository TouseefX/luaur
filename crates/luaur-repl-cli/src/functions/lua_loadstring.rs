use alloc::string::String;
use core::ffi::c_char;

use luaur_ast::records::parse_options::ParseOptions;
use luaur_bytecode::records::bytecode_encoder::BytecodeEncoder;
use luaur_compiler::functions::compile::compile;
use luaur_vm::functions::lua_insert::lua_insert;
use luaur_vm::functions::lua_l_checklstring::lua_l_checklstring;
use luaur_vm::functions::lua_l_optlstring::lua_l_optlstring;
use luaur_vm::functions::lua_pushnil::lua_pushnil;
use luaur_vm::functions::lua_setsafeenv::lua_setsafeenv;
use luaur_vm::functions::luau_load::luau_load;
use luaur_vm::macros::lua_environindex::LUA_ENVIRONINDEX;
use luaur_vm::type_aliases::lua_state::lua_State;

use crate::functions::copts::copts;

pub unsafe fn lua_loadstring(l: *mut lua_State) -> i32 {
    let mut len: usize = 0;
    let s = lua_l_checklstring(l, 1, &mut len as *mut usize);
    let chunkname = lua_l_optlstring(l, 2, s, core::ptr::null_mut());

    lua_setsafeenv(l, LUA_ENVIRONINDEX, false as core::ffi::c_int);

    let source_bytes = core::slice::from_raw_parts(s as *const u8, len);
    let source: String = core::str::from_utf8_unchecked(source_bytes).into();

    struct NoopEncoder;
    impl BytecodeEncoder for NoopEncoder {
        fn encode(&mut self, _data: &mut [u32]) {}
    }
    let options = copts();
    let parse_options = ParseOptions::default();
    let mut encoder = NoopEncoder;
    let bytecode = compile(
        &source,
        &options,
        &parse_options,
        &mut encoder as *mut dyn BytecodeEncoder,
    );

    if luau_load(
        l,
        chunkname,
        bytecode.as_ptr() as *const c_char,
        bytecode.len(),
        0,
    ) == 0
    {
        return 1;
    }

    lua_pushnil(l);
    lua_insert(l, -2); // put before error message
    2 // return nil plus error message
}
