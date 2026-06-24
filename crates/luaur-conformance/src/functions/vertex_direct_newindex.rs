use core::ffi::{c_int, c_void, CStr};

use luaur_vm::functions::lua_l_checkvector::lua_l_checkvector;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;
use luaur_vm::records::lua_state::lua_State;

use crate::enums::direct_slot::DirectSlot;
use crate::functions::lua_vec_2_get::lua_vec_2_get;
use crate::functions::update_direct_slot::updateDirectSlot;
use crate::records::vertex::Vertex;

#[allow(non_snake_case)]
pub unsafe extern "C" fn vertexDirectNewindex(
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
        Some(DirectSlot::Pos) => {
            let pos = lua_l_checkvector(L, 3);
            (*self_ptr).pos[0] = *pos.add(0);
            (*self_ptr).pos[1] = *pos.add(1);
            (*self_ptr).pos[2] = *pos.add(2);
        }
        Some(DirectSlot::Normal) => {
            let normal = lua_l_checkvector(L, 3);
            (*self_ptr).normal[0] = *normal.add(0);
            (*self_ptr).normal[1] = *normal.add(1);
            (*self_ptr).normal[2] = *normal.add(2);
        }
        Some(DirectSlot::UV) => {
            let uv = lua_vec_2_get(L, 3);
            (*self_ptr).uv[0] = (*uv).x;
            (*self_ptr).uv[1] = (*uv).y;
        }
        _ => {
            let name_ptr = luaL_checkstring!(L, 2);
            let name = CStr::from_ptr(name_ptr).to_string_lossy();
            lua_l_error_l(
                L,
                c"%s is not a writable member of vertex".as_ptr(),
                format_args!("{name} is not a writable member of vertex"),
            );
        }
    }
}
