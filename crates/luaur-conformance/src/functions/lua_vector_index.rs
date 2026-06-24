use core::ffi::{c_int, CStr};

use luaur_vm::functions::lua_l_checkvector::lua_l_checkvector;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_pushcclosurek::lua_pushcclosurek;
use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
use luaur_vm::functions::lua_pushvector_lapi::lua_pushvector_lua_state_f32_f32_f32_f32;
use luaur_vm::functions::lua_pushvector_lapi_alt_b::lua_pushvector_lua_state_f32_f32_f32;
use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;
use luaur_vm::macros::lua_vector_size::LUA_VECTOR_SIZE;
use luaur_vm::records::lua_state::lua_State;
use luaur_vm::type_aliases::lua_c_function::lua_CFunction;

use crate::functions::lua_vector_dot::lua_vector_dot;

pub unsafe fn lua_vector_index(L: *mut lua_State) -> c_int {
    let v = lua_l_checkvector(L, 1);
    let name_ptr = luaL_checkstring!(L, 2);
    let name = CStr::from_ptr(name_ptr).to_str().unwrap_or("");

    if name == "Magnitude" {
        let mut sum = *v.add(0) * *v.add(0) + *v.add(1) * *v.add(1) + *v.add(2) * *v.add(2);
        if LUA_VECTOR_SIZE == 4 {
            sum += *v.add(3) * *v.add(3);
        }
        lua_pushnumber(L, sum.sqrt() as f64);
        return 1;
    }

    if name == "Unit" {
        let mut sum = *v.add(0) * *v.add(0) + *v.add(1) * *v.add(1) + *v.add(2) * *v.add(2);
        if LUA_VECTOR_SIZE == 4 {
            sum += *v.add(3) * *v.add(3);
        }
        let inv_sqrt = 1.0 / sum.sqrt();

        if LUA_VECTOR_SIZE == 4 {
            lua_pushvector_lua_state_f32_f32_f32_f32(
                L,
                *v.add(0) * inv_sqrt,
                *v.add(1) * inv_sqrt,
                *v.add(2) * inv_sqrt,
                *v.add(3) * inv_sqrt,
            );
        } else {
            lua_pushvector_lua_state_f32_f32_f32(
                L,
                *v.add(0) * inv_sqrt,
                *v.add(1) * inv_sqrt,
                *v.add(2) * inv_sqrt,
            );
        }
        return 1;
    }

    if name == "Dot" {
        let dot: lua_CFunction = Some(core::mem::transmute(
            lua_vector_dot as fn(*mut lua_State) -> i32,
        ));
        lua_pushcclosurek(L, dot, c"Dot".as_ptr(), 0, None);
        return 1;
    }

    lua_l_error_l(
        L,
        c"%s is not a valid member of vector".as_ptr(),
        format_args!("{name} is not a valid member of vector"),
    );
    0
}
