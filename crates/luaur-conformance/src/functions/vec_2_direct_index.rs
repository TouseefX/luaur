use core::ffi::{c_int, c_void};

use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;
use luaur_vm::records::lua_state::lua_State;

use crate::enums::direct_slot::DirectSlot;
use crate::functions::lua_vec_2_push::lua_vec_2_push;
use crate::functions::update_direct_slot::updateDirectSlot;
use crate::records::vec_2_conformance_ir_hooks::Vec2;

#[allow(non_snake_case)]
pub unsafe extern "C" fn vec2DirectIndex(
    L: *mut lua_State,
    data: *mut c_void,
    atom: c_int,
    cachedslot: *mut u16,
    _utag: c_int,
) {
    let self_ptr = data as *mut Vec2;

    if *cachedslot == 0 {
        updateDirectSlot(atom, cachedslot);
    }

    match DirectSlot::from_u16(*cachedslot) {
        Some(DirectSlot::X) => lua_pushnumber(L, (*self_ptr).x as f64),
        Some(DirectSlot::Y) => lua_pushnumber(L, (*self_ptr).y as f64),
        Some(DirectSlot::Magnitude) => lua_pushnumber(
            L,
            ((*self_ptr).x * (*self_ptr).x + (*self_ptr).y * (*self_ptr).y).sqrt() as f64,
        ),
        Some(DirectSlot::Unit) => {
            let inv = 1.0 / ((*self_ptr).x * (*self_ptr).x + (*self_ptr).y * (*self_ptr).y).sqrt();
            let result = lua_vec_2_push(L);
            (*result).x = (*self_ptr).x * inv;
            (*result).y = (*self_ptr).y * inv;
        }
        Some(DirectSlot::Sizeof) => lua_pushnumber(L, core::mem::size_of::<Vec2>() as f64),
        _ => {
            let name_ptr = luaL_checkstring!(L, 2);
            let name = core::ffi::CStr::from_ptr(name_ptr).to_string_lossy();
            lua_l_error_l(
                L,
                c"%s is not a valid member of vec2".as_ptr(),
                format_args!("{name} is not a valid member of vec2"),
            );
        }
    }
}
