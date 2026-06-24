use core::ffi::{c_int, c_void};

use luaur_vm::functions::lua_userdatadirectfield_setboolean::lua_userdatadirectfield_setboolean;

use crate::records::vec_2_direct_field_access_test::Vec2;

pub unsafe extern "C" fn direct_field_access_get_non_zero_boolean(
    ud: *mut c_void,
    result: *mut c_void,
) {
    let vec = &*(ud as *mut Vec2);
    let non_zero = (vec.x != 0.0 || vec.y != 0.0) as c_int;
    lua_userdatadirectfield_setboolean(result, non_zero);
}
