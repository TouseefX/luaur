use core::ffi::{c_int, c_void, CStr};

use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_namecallatom::lua_namecallatom;
use luaur_vm::records::lua_state::lua_State;

use crate::enums::direct_slot::DirectSlot;
use crate::functions::lua_vec_2_clone::lua_vec_2_clone;
use crate::functions::lua_vec_2_dot::lua_vec_2_dot;
use crate::functions::lua_vec_2_min::lua_vec_2_min;
use crate::functions::lua_vec_2_reenter::lua_vec_2_reenter;
use crate::functions::update_direct_slot::updateDirectSlot;
use crate::records::vec_2_conformance_ir_hooks::Vec2;

#[allow(non_snake_case)]
pub unsafe extern "C" fn vec2DirectNamecall(
    L: *mut lua_State,
    data: *mut c_void,
    atom: c_int,
    cachedslot: *mut u16,
    _utag: c_int,
) -> c_int {
    let self_ptr = data as *mut Vec2;

    if *cachedslot == 0 {
        updateDirectSlot(atom, cachedslot);
    }

    match DirectSlot::from_u16(*cachedslot) {
        Some(DirectSlot::Dot) => lua_vec_2_dot(L, self_ptr),
        Some(DirectSlot::Min) => lua_vec_2_min(L, self_ptr),
        Some(DirectSlot::Clone) => lua_vec_2_clone(L, self_ptr),
        Some(DirectSlot::Reenter) => lua_vec_2_reenter(L, self_ptr),
        _ => {
            let method = lua_namecallatom(L, core::ptr::null_mut());
            let method = if method.is_null() {
                ""
            } else {
                CStr::from_ptr(method).to_str().unwrap_or("")
            };
            lua_l_error_l(
                L,
                c"%s is not a valid method of vec2".as_ptr(),
                format_args!("{method} is not a valid method of vec2"),
            );
            0
        }
    }
}
