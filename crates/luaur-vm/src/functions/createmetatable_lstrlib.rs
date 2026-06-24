use crate::functions::lua_createtable::lua_createtable;
use crate::functions::lua_pushvalue::lua_pushvalue;
use crate::functions::lua_setfield::lua_setfield;
use crate::functions::lua_setmetatable::lua_setmetatable;
use crate::macros::lua_pop::lua_pop;
use crate::macros::lua_pushliteral::LUA_PUSHLITERAL;
use crate::type_aliases::lua_state::lua_State;
use core::ffi::c_int;

#[allow(non_snake_case)]
pub unsafe fn createmetatable_mut(l: *mut lua_State) {
    // The dependency cards show lua_createtable, lua_pushvalue, lua_setmetatable, and lua_setfield
    // as having 0-arg stubs. To call them with the correct arguments as per the C++ source,
    // we use transmute to cast the function pointers to the correct signatures.

    type LuaCreateTableFn = unsafe extern "C" fn(*mut lua_State, c_int, c_int);
    let lua_createtable_ptr: LuaCreateTableFn = core::mem::transmute(lua_createtable as *const ());
    lua_createtable_ptr(l, 0, 1); // create metatable for strings

    LUA_PUSHLITERAL(l as *mut core::ffi::c_void, c"".as_ptr()); // dummy string

    type LuaPushValueFn = unsafe extern "C" fn(*mut lua_State, c_int);
    let lua_pushvalue_ptr: LuaPushValueFn = core::mem::transmute(lua_pushvalue as *const ());
    lua_pushvalue_ptr(l, -2);

    type LuaSetMetatableFn = unsafe extern "C" fn(*mut lua_State, c_int) -> c_int;
    let lua_setmetatable_ptr: LuaSetMetatableFn =
        core::mem::transmute(lua_setmetatable as *const ());
    lua_setmetatable_ptr(l, -2); // set string metatable

    lua_pop(l, 1); // pop dummy string

    lua_pushvalue_ptr(l, -2); // string library...

    type LuaSetFieldFn = unsafe extern "C" fn(*mut lua_State, c_int, *const core::ffi::c_char);
    let lua_setfield_ptr: LuaSetFieldFn = core::mem::transmute(lua_setfield as *const ());
    lua_setfield_ptr(l, -2, c"__index".as_ptr()); // ...is the __index metamethod

    lua_pop(l, 1); // pop metatable
}
