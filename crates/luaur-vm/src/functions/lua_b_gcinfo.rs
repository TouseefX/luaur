use crate::enums::lua_gc_op::lua_GCOp;
use crate::functions::lua_gc::lua_gc;
use crate::functions::lua_pushinteger::lua_pushinteger;
use crate::type_aliases::lua_state::lua_State;

#[no_mangle]
pub unsafe fn lua_b_gcinfo(l: *mut lua_State) -> i32 {
    lua_pushinteger(l, lua_gc(l, lua_GCOp::LUA_GCCOUNT as i32, 0));
    1
}
