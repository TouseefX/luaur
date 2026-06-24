//! `lua_State` as seen by CodeGen — the VM's own type, not an opaque twin
//! (CodeGen executes against real VM state).

pub use luaur_vm::records::lua_state::lua_State;

#[allow(non_camel_case_types)]
pub type lua_state = lua_State;

pub type LuaState = lua_State;
