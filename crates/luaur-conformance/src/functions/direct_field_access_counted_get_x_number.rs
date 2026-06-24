use core::ffi::c_void;

use luaur_vm::functions::lua_userdatadirectfield_setnumber::lua_userdatadirectfield_setnumber;

use crate::functions::direct_field_access_increment_handler_hit_count::direct_field_access_increment_handler_hit_count;
use crate::records::vec_2_direct_field_access_test::Vec2;

pub unsafe extern "C" fn direct_field_access_counted_get_x_number(
    ud: *mut c_void,
    result: *mut c_void,
) {
    direct_field_access_increment_handler_hit_count();
    lua_userdatadirectfield_setnumber(result, (*(ud as *mut Vec2)).x);
}
