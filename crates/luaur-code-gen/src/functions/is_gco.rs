use crate::macros::codegen_assert::CODEGEN_ASSERT;
use luaur_vm::enums::lua_type::{lua_Type, LUA_T_COUNT};

pub fn is_gco(tag: u8) -> bool {
    CODEGEN_ASSERT!(tag < LUA_T_COUNT as u8);

    // mirrors iscollectable(o) from VM/lobject.h
    tag >= lua_Type::LUA_TSTRING as u8
}
