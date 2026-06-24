use crate::macros::setnvalue::setnvalue;
use crate::type_aliases::t_value::TValue;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

#[no_mangle]
pub unsafe fn lua_userdatadirectfield_setnumber(result: *mut core::ffi::c_void, n: f64) {
    LUAU_ASSERT!(luaur_common::FFlag::LuauDirectFieldGet.get());
    setnvalue!(result as *mut TValue, n);
}
