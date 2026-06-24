use crate::functions::get_type_function_runtime::get_type_function_runtime;
use crate::type_aliases::lua_state::lua_State;
use alloc::string::String;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_l_tolstring::lua_l_tolstring;
use luaur_vm::macros::lua_pop::lua_pop;

pub unsafe fn print(l: *mut lua_State) -> core::ffi::c_int {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let mut result = String::new();

    let n = lua_gettop(vm_l);
    for i in 1..=n {
        let mut len = 0usize;
        let s = lua_l_tolstring(vm_l, i, &mut len as *mut usize);
        if i > 1 {
            result.push('\t');
        }

        let bytes = core::slice::from_raw_parts(s as *const u8, len);
        result.push_str(&String::from_utf8_lossy(bytes));
        lua_pop(vm_l, 1);
    }

    let ctx = get_type_function_runtime(l);
    (*ctx).messages.push(result);

    0
}
