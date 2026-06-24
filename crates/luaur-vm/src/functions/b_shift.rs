use crate::functions::lua_pushunsigned::lua_pushunsigned;
use crate::macros::nbits::NBITS;
use crate::macros::trim::trim;
use crate::type_aliases::b_uint::b_uint;
use crate::type_aliases::lua_state::lua_State;

pub fn b_shift(l: *mut lua_State, mut r: b_uint, mut i: core::ffi::c_int) -> core::ffi::c_int {
    // Mirrors VM/src/lbitlib.cpp:b_shift
    if i < 0 {
        i = -i;
        r = trim(r);
        if i >= NBITS as core::ffi::c_int {
            r = 0;
        } else {
            r >>= i as u32;
        }
    } else {
        if i >= NBITS as core::ffi::c_int {
            r = 0;
        } else {
            r <<= i as u32;
        }
        r = trim(r);
    }

    lua_pushunsigned(l, r);
    1
}
