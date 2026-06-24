use core::ffi::c_int;
use luaur_vm::enums::lua_gc_op::lua_GCOp;
use luaur_vm::functions::lua_gc::lua_gc;
use luaur_vm::functions::lua_l_checkoption::lua_l_checkoption;
use luaur_vm::functions::lua_l_optinteger::lua_l_optinteger;
use luaur_vm::functions::lua_pushboolean::lua_pushboolean;
use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub fn lua_collectgarbage(L: *mut lua_State) -> i32 {
    let opts = [
        "stop\0",
        "restart\0",
        "collect\0",
        "count\0",
        "isrunning\0",
        "step\0",
        "setgoal\0",
        "setstepmul\0",
        "setstepsize\0",
        "\0",
    ];

    let opts_ptrs = [
        opts[0].as_ptr() as *const i8,
        opts[1].as_ptr() as *const i8,
        opts[2].as_ptr() as *const i8,
        opts[3].as_ptr() as *const i8,
        opts[4].as_ptr() as *const i8,
        opts[5].as_ptr() as *const i8,
        opts[6].as_ptr() as *const i8,
        opts[7].as_ptr() as *const i8,
        opts[8].as_ptr() as *const i8,
        core::ptr::null(),
    ];

    let optsnum = [
        lua_GCOp::LUA_GCSTOP as c_int,
        lua_GCOp::LUA_GCRESTART as c_int,
        lua_GCOp::LUA_GCCOLLECT as c_int,
        lua_GCOp::LUA_GCCOUNT as c_int,
        lua_GCOp::LUA_GCISRUNNING as c_int,
        lua_GCOp::LUA_GCSTEP as c_int,
        lua_GCOp::LUA_GCSETGOAL as c_int,
        lua_GCOp::LUA_GCSETSTEPMUL as c_int,
        lua_GCOp::LUA_GCSETSTEPSIZE as c_int,
    ];

    let o = lua_l_checkoption(L, 1, Some("collect"), opts_ptrs.as_ptr());
    let ex = lua_l_optinteger(L, 2, 0);
    let res = lua_gc(L, optsnum[o as usize], ex);

    match optsnum[o as usize] {
        x if x == lua_GCOp::LUA_GCSTEP as c_int || x == lua_GCOp::LUA_GCISRUNNING as c_int => {
            unsafe {
                lua_pushboolean(L, res);
            }
            1
        }
        _ => {
            unsafe {
                lua_pushnumber(L, res as f64);
            }
            1
        }
    }
}
