use core::ffi::c_int;

use luaur_vm::functions::lua_l_newmetatable::lua_l_newmetatable;
use luaur_vm::functions::lua_pushcclosurek::lua_pushcclosurek;
use luaur_vm::functions::lua_pushvalue::lua_pushvalue;
use luaur_vm::functions::lua_setfield::lua_setfield;
use luaur_vm::functions::lua_setreadonly::lua_setreadonly;
use luaur_vm::functions::lua_setuserdatametatable::lua_setuserdatametatable;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::macros::lua_setglobal::lua_setglobal;
use luaur_vm::records::lua_state::lua_State;
use luaur_vm::type_aliases::lua_c_function::lua_CFunction;

use crate::functions::lua_vec_2::lua_vec_2;
use crate::functions::lua_vec_2_get::lua_vec_2_get;
use crate::functions::lua_vec_2_index::lua_vec_2_index;
use crate::functions::lua_vec_2_namecall::lua_vec_2_namecall;
use crate::functions::lua_vec_2_newindex::lua_vec_2_newindex;
use crate::functions::lua_vec_2_push::{kTagVec2, lua_vec_2_push};
use crate::functions::lua_vertex::lua_vertex;
use crate::functions::lua_vertex_index::lua_vertex_index;
use crate::functions::lua_vertex_namecall::lua_vertex_namecall;
use crate::functions::lua_vertex_newindex::lua_vertex_newindex;
use crate::functions::lua_vertex_push::kTagVertex;

unsafe fn as_c_function(f: unsafe fn(*mut lua_State) -> c_int) -> lua_CFunction {
    Some(core::mem::transmute(f))
}

unsafe extern "C" fn vec2_add(L: *mut lua_State) -> c_int {
    let a = lua_vec_2_get(L, 1);
    let b = lua_vec_2_get(L, 2);
    let data = lua_vec_2_push(L);
    (*data).x = (*a).x + (*b).x;
    (*data).y = (*a).y + (*b).y;
    1
}

unsafe extern "C" fn vec2_sub(L: *mut lua_State) -> c_int {
    let a = lua_vec_2_get(L, 1);
    let b = lua_vec_2_get(L, 2);
    let data = lua_vec_2_push(L);
    (*data).x = (*a).x - (*b).x;
    (*data).y = (*a).y - (*b).y;
    1
}

unsafe extern "C" fn vec2_mul(L: *mut lua_State) -> c_int {
    let a = lua_vec_2_get(L, 1);
    let b = lua_vec_2_get(L, 2);
    let data = lua_vec_2_push(L);
    (*data).x = (*a).x * (*b).x;
    (*data).y = (*a).y * (*b).y;
    1
}

unsafe extern "C" fn vec2_div(L: *mut lua_State) -> c_int {
    let a = lua_vec_2_get(L, 1);
    let b = lua_vec_2_get(L, 2);
    let data = lua_vec_2_push(L);
    (*data).x = (*a).x / (*b).x;
    (*data).y = (*a).y / (*b).y;
    1
}

unsafe extern "C" fn vec2_unm(L: *mut lua_State) -> c_int {
    let a = lua_vec_2_get(L, 1);
    let data = lua_vec_2_push(L);
    (*data).x = -(*a).x;
    (*data).y = -(*a).y;
    1
}

#[allow(non_snake_case)]
pub unsafe fn setupUserdataHelpers(L: *mut lua_State) {
    lua_l_newmetatable(L, c"vec2".as_ptr());
    lua_pushvalue(L, -1);
    lua_setuserdatametatable(L, kTagVec2);

    lua_pushcclosurek(
        L,
        as_c_function(lua_vec_2_index),
        core::ptr::null(),
        0,
        None,
    );
    lua_setfield(L, -2, c"__index".as_ptr());

    lua_pushcclosurek(
        L,
        as_c_function(lua_vec_2_newindex),
        core::ptr::null(),
        0,
        None,
    );
    lua_setfield(L, -2, c"__newindex".as_ptr());

    lua_pushcclosurek(
        L,
        as_c_function(lua_vec_2_namecall),
        core::ptr::null(),
        0,
        None,
    );
    lua_setfield(L, -2, c"__namecall".as_ptr());

    lua_pushcclosurek(
        L,
        Some(core::mem::transmute(
            vec2_add as unsafe extern "C" fn(*mut lua_State) -> c_int,
        )),
        core::ptr::null(),
        0,
        None,
    );
    lua_setfield(L, -2, c"__add".as_ptr());

    lua_pushcclosurek(
        L,
        Some(core::mem::transmute(
            vec2_sub as unsafe extern "C" fn(*mut lua_State) -> c_int,
        )),
        core::ptr::null(),
        0,
        None,
    );
    lua_setfield(L, -2, c"__sub".as_ptr());

    lua_pushcclosurek(
        L,
        Some(core::mem::transmute(
            vec2_mul as unsafe extern "C" fn(*mut lua_State) -> c_int,
        )),
        core::ptr::null(),
        0,
        None,
    );
    lua_setfield(L, -2, c"__mul".as_ptr());

    lua_pushcclosurek(
        L,
        Some(core::mem::transmute(
            vec2_div as unsafe extern "C" fn(*mut lua_State) -> c_int,
        )),
        core::ptr::null(),
        0,
        None,
    );
    lua_setfield(L, -2, c"__div".as_ptr());

    lua_pushcclosurek(
        L,
        Some(core::mem::transmute(
            vec2_unm as unsafe extern "C" fn(*mut lua_State) -> c_int,
        )),
        core::ptr::null(),
        0,
        None,
    );
    lua_setfield(L, -2, c"__unm".as_ptr());

    lua_setreadonly(L, -1, 1);

    lua_pushcclosurek(
        L,
        Some(core::mem::transmute(lua_vec_2 as fn(*mut lua_State) -> i32)),
        c"vec2".as_ptr(),
        0,
        None,
    );
    lua_setglobal(L, c"vec2".as_ptr());

    lua_pop(L, 1);

    lua_l_newmetatable(L, c"vertex".as_ptr());
    lua_pushvalue(L, -1);
    lua_setuserdatametatable(L, kTagVertex as c_int);

    lua_pushcclosurek(
        L,
        Some(core::mem::transmute(
            lua_vertex_index as unsafe extern "C" fn(*mut lua_State) -> c_int,
        )),
        core::ptr::null(),
        0,
        None,
    );
    lua_setfield(L, -2, c"__index".as_ptr());

    lua_pushcclosurek(
        L,
        as_c_function(lua_vertex_newindex),
        core::ptr::null(),
        0,
        None,
    );
    lua_setfield(L, -2, c"__newindex".as_ptr());

    lua_pushcclosurek(
        L,
        as_c_function(lua_vertex_namecall),
        core::ptr::null(),
        0,
        None,
    );
    lua_setfield(L, -2, c"__namecall".as_ptr());

    lua_setreadonly(L, -1, 1);

    lua_pushcclosurek(
        L,
        Some(core::mem::transmute(
            lua_vertex as fn(*mut lua_State) -> i32,
        )),
        c"vertex".as_ptr(),
        0,
        None,
    );
    lua_setglobal(L, c"vertex".as_ptr());

    lua_pop(L, 1);
}
