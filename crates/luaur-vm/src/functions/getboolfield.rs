use crate::functions::lua_rawgetfield::lua_rawgetfield;
use crate::functions::lua_toboolean::lua_toboolean;
use crate::macros::lua_pop::lua_pop;
use crate::type_aliases::lua_state::lua_State;
use core::ffi::{c_char, c_int};

pub fn getboolfield(L: *mut lua_State, key: &str) -> i32 {
    let key_bytes = key.as_bytes();

    // lua_rawgetfield expects a null-terminated C string key.
    let mut buf = key_bytes.to_vec();
    buf.push(0);
    let key_c: *const c_char = buf.as_ptr() as *const c_char;

    unsafe {
        // The skeleton stubs for lua_rawgetfield, lua_toboolean, and lua_type currently have 0-arity signatures.
        // To allow this file to compile while those stubs exist, we must transmute the function pointers
        // to the correct signatures expected by the C++ logic.
        let raw_get_field: unsafe extern "C" fn(*mut lua_State, c_int, *const c_char) =
            core::mem::transmute(lua_rawgetfield as *const ());
        raw_get_field(L, -1, key_c);

        // We cannot use the lua_isnil! macro because it calls the 0-arity lua_type stub directly,
        // which causes a compilation error. We manually implement the logic here using a transmuted lua_type.
        let lua_type_ptr: unsafe extern "C" fn(*mut lua_State, c_int) -> c_int =
            core::mem::transmute(crate::functions::lua_type::lua_type as *const ());

        let is_nil = lua_type_ptr(L, -1) == (crate::enums::lua_type::lua_Type::LUA_TNIL as i32);

        let res: c_int = if is_nil {
            -1
        } else {
            let to_boolean: unsafe extern "C" fn(*mut lua_State, c_int) -> c_int =
                core::mem::transmute(lua_toboolean as *const ());
            to_boolean(L, -1)
        };

        lua_pop(L, 1);
        res
    }
}
