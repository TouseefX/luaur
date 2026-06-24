use core::ffi::{c_int, c_void, CStr};

use luaur_vm::functions::lua_l_checknumber::luaL_checknumber;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;
use luaur_vm::records::lua_state::lua_State;

use crate::enums::direct_slot::DirectSlot;
use crate::functions::update_direct_slot::updateDirectSlot;
use crate::records::vec_2_conformance_ir_hooks::Vec2;

#[allow(non_snake_case)]
pub unsafe extern "C" fn vec2DirectNewindex(
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
        Some(DirectSlot::X) => (*self_ptr).x = luaL_checknumber(L, 3) as f32,
        Some(DirectSlot::Y) => (*self_ptr).y = luaL_checknumber(L, 3) as f32,
        _ => {
            let name_ptr = luaL_checkstring!(L, 2);
            let name = CStr::from_ptr(name_ptr).to_string_lossy();
            lua_l_error_l(
                L,
                c"%s is not a writable member of vec2".as_ptr(),
                format_args!("{name} is not a writable member of vec2"),
            );
        }
    }
}
