use core::ffi::{c_int, c_void, CStr};

use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_namecallatom::lua_namecallatom;
use luaur_vm::records::lua_state::lua_State;

use crate::enums::direct_slot::DirectSlot;
use crate::functions::lua_vertex_clone::lua_vertex_clone;
use crate::functions::update_direct_slot::updateDirectSlot;
use crate::records::vertex::Vertex;

#[allow(non_snake_case)]
pub unsafe extern "C" fn vertex_direct_namecall(
    L: *mut lua_State,
    data: *mut c_void,
    atom: c_int,
    cachedslot: *mut u16,
    _utag: c_int,
) -> c_int {
    let self_ptr = data as *mut Vertex;

    if *cachedslot == 0 {
        updateDirectSlot(atom, cachedslot);
    }

    match DirectSlot::from_u16(*cachedslot) {
        Some(DirectSlot::Clone) => lua_vertex_clone(L, self_ptr),
        _ => {
            let method = lua_namecallatom(L, core::ptr::null_mut());
            let method = if method.is_null() {
                ""
            } else {
                CStr::from_ptr(method).to_str().unwrap_or("")
            };
            lua_l_error_l(
                L,
                c"%s is not a valid method of vertex".as_ptr(),
                format_args!("{method} is not a valid method of vertex"),
            );
            0
        }
    }
}
