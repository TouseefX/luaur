use core::ffi::c_void;

use luaur_vm::functions::lua_userdatadirectfield_setnumber::lua_userdatadirectfield_setnumber;

use crate::functions::direct_field_access_increment_handler_hit_count::direct_field_access_increment_handler_hit_count;

pub unsafe extern "C" fn direct_field_access_counted_get_999_number(
    _ud: *mut c_void,
    result: *mut c_void,
) {
    lua_userdatadirectfield_setnumber(result, 999.0);
    direct_field_access_increment_handler_hit_count();
}
