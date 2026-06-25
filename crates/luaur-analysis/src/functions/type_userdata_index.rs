use crate::functions::get_tag::get_tag;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use luaur_vm::functions::lua_getfield::lua_getfield;
use luaur_vm::functions::lua_pushlstring::lua_pushlstring;
use luaur_vm::functions::lua_pushvalue::lua_pushvalue;
use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;
use luaur_vm::macros::lua_upvalueindex::lua_upvalueindex;

use core::ffi::CStr;

pub unsafe fn type_userdata_index(l: *mut lua_State) -> core::ffi::c_int {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let self_ty = get_type_user_data(l, 1);
    let field_ptr = luaL_checkstring!(vm_l, 2);
    let field = CStr::from_ptr(field_ptr).to_bytes();

    if field == b"tag" {
        let tag = get_tag(l, self_ty);
        // `tag` is an owned Rust String with no trailing NUL. Pushing it through
        // the C-string lua_pushstring scanned past its bytes into adjacent memory,
        // yielding garbage like "table\u{7f}" (nondeterministic — passed locally,
        // failed in CI; type_function_user_tag_field). Use the length-aware
        // lua_pushlstring to copy exactly tag.len() bytes.
        lua_pushlstring(vm_l, tag.as_ptr() as *const core::ffi::c_char, tag.len());
        1
    } else {
        let pushvalue: unsafe extern "C" fn(
            *mut luaur_vm::records::lua_state::lua_State,
            core::ffi::c_int,
        ) = core::mem::transmute(lua_pushvalue as *const ());
        pushvalue(vm_l, lua_upvalueindex(1));
        let getfield: unsafe extern "C" fn(
            *mut luaur_vm::records::lua_state::lua_State,
            core::ffi::c_int,
            *const core::ffi::c_char,
        ) -> core::ffi::c_int = core::mem::transmute(lua_getfield as *const ());
        getfield(vm_l, -1, field_ptr as *const core::ffi::c_char);
        1
    }
}
