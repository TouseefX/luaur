use core::ffi::c_int;

use luaur_vm::functions::lua_resumeerror::lua_resumeerror;
use luaur_vm::functions::lua_tothread::lua_tothread;
use luaur_vm::functions::lua_xmove::lua_xmove;
use luaur_vm::records::lua_state::lua_State;

pub unsafe fn conformance_p_call_resume_error(l: *mut lua_State) -> c_int {
    let co = lua_tothread(l, 1);
    lua_xmove(l, co, 1);
    lua_resumeerror(co, l);
    0
}
