use core::ffi::{c_int, c_void};

use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
use luaur_vm::functions::lua_pushvector_lapi_alt_b::lua_pushvector_lua_state_f32_f32_f32;
use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;
use luaur_vm::records::lua_state::lua_State;

use crate::enums::direct_slot::DirectSlot;
use crate::functions::lua_vec_2_push::lua_vec_2_push;
use crate::functions::update_direct_slot::updateDirectSlot;
use crate::records::vertex::Vertex;

#[allow(non_snake_case)]
pub unsafe extern "C" fn vertex_direct_index(
    L: *mut lua_State,
    data: *mut c_void,
    atom: c_int,
    cachedslot: *mut u16,
    _utag: c_int,
) {
    let self_ptr = data as *mut Vertex;

    if *cachedslot == 0 {
        updateDirectSlot(atom, cachedslot);
    }

    match DirectSlot::from_u16(*cachedslot) {
        Some(DirectSlot::Pos) => lua_pushvector_lua_state_f32_f32_f32(
            L,
            (*self_ptr).pos[0],
            (*self_ptr).pos[1],
            (*self_ptr).pos[2],
        ),
        Some(DirectSlot::Normal) => lua_pushvector_lua_state_f32_f32_f32(
            L,
            (*self_ptr).normal[0],
            (*self_ptr).normal[1],
            (*self_ptr).normal[2],
        ),
        Some(DirectSlot::UV) => {
            let uv = lua_vec_2_push(L);
            (*uv).x = (*self_ptr).uv[0];
            (*uv).y = (*self_ptr).uv[1];
        }
        Some(DirectSlot::Sizeof) => lua_pushnumber(L, core::mem::size_of::<Vertex>() as f64),
        _ => {
            let name_ptr = luaL_checkstring!(L, 2);
            let name = core::ffi::CStr::from_ptr(name_ptr).to_string_lossy();
            lua_l_error_l(
                L,
                c"%s is not a valid member of vertex".as_ptr(),
                format_args!("{name} is not a valid member of vertex"),
            );
        }
    }
}
