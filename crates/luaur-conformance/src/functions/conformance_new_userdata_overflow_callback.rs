use core::ffi::c_int;

use luaur_vm::functions::lua_getmetatable::lua_getmetatable;
use luaur_vm::functions::lua_newuserdatadtor::lua_newuserdatadtor;
use luaur_vm::records::lua_state::lua_State;

use crate::functions::conformance_new_userdata_overflow_dtor::conformance_new_userdata_overflow_dtor;

pub unsafe fn conformance_new_userdata_overflow_callback(l: *mut lua_State) -> c_int {
    lua_newuserdatadtor(l, usize::MAX, Some(conformance_new_userdata_overflow_dtor));
    lua_getmetatable(l, -1);

    0
}
