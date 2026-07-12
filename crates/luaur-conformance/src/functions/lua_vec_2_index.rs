use core::ffi::CStr;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;
use luaur_vm::records::lua_state::lua_State;

use crate::functions::lua_vec_2_get::lua_vec_2_get;
use crate::functions::lua_vec_2_push::lua_vec_2_push;
use crate::records::vec_2_conformance_ir_hooks::Vec2;

#[allow(non_snake_case)]
pub unsafe fn lua_vec_2_index(L: *mut lua_State) -> i32 {
    let v = lua_vec_2_get(L, 1);
    let name_ptr = luaL_checkstring!(L, 2);
    let name = CStr::from_ptr(name_ptr).to_str().unwrap_or("");

    if name == "X" {
        lua_pushnumber(L, (*v).x as f64);
        return 1;
    }

    if name == "Y" {
        lua_pushnumber(L, (*v).y as f64);
        return 1;
    }

    if name == "Magnitude" {
        let mag = ((*v).x * (*v).x + (*v).y * (*v).y).sqrt();
        lua_pushnumber(L, mag as f64);
        return 1;
    }

    if name == "Unit" {
        let mag_sq = (*v).x * (*v).x + (*v).y * (*v).y;
        let inv_sqrt = 1.0 / mag_sq.sqrt();

        let data = lua_vec_2_push(L);
        (*data).x = (*v).x * inv_sqrt;
        (*data).y = (*v).y * inv_sqrt;
        return 1;
    }

    if name == "sizeof" {
        lua_pushnumber(L, core::mem::size_of::<Vec2>() as f64);
        return 1;
    }

    lua_l_error_l(
        L,
        "{} is not a valid member of vector\0".as_ptr() as *const core::ffi::c_char,
        core::format_args!("{} is not a valid member of vector", name),
    );
    0
}
