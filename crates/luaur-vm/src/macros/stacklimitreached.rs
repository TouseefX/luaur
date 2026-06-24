use crate::records::lua_state::lua_State;
use crate::type_aliases::t_value::TValue;

#[allow(non_snake_case)]
pub fn stacklimitreached(L: *mut lua_State, n: core::ffi::c_int) -> bool {
    unsafe {
        let stack_last = (*L).stack_last as *mut core::ffi::c_char;
        let top = (*L).top as *mut core::ffi::c_char;
        let diff = stack_last as usize - top as usize;
        let threshold = (n as usize) * (core::mem::size_of::<TValue>() as usize);
        diff <= threshold
    }
}
