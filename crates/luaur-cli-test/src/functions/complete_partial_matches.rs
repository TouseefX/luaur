use alloc::string::{String, ToString};
use luaur_vm::records::lua_state::lua_State;

pub fn complete_partial_matches(
    l: *mut lua_State,
    complete_only_functions: bool,
    edit_buffer: &String,
    prefix: &str,
    add_completion_callback: &mut dyn FnMut(&str, &str),
) {
    unsafe {
        for _ in 0..50 {
            if luaur_vm::functions::lua_type::lua_type(l, -1)
                != luaur_vm::enums::lua_type::lua_Type::LUA_TTABLE as i32
            {
                break;
            }

            luaur_vm::functions::lua_pushnil::lua_pushnil(l);

            while luaur_vm::functions::lua_next::lua_next(l, -2) != 0 {
                if luaur_vm::functions::lua_type::lua_type(l, -2)
                    == luaur_vm::enums::lua_type::lua_Type::LUA_TSTRING as i32
                {
                    let key_ptr = luaur_vm::functions::lua_tolstring::lua_tolstring(
                        l,
                        -2,
                        core::ptr::null_mut(),
                    );
                    let key = core::ffi::CStr::from_ptr(key_ptr).to_string_lossy();
                    let value_type = luaur_vm::functions::lua_type::lua_type(l, -1);
                    let required_value_type = !complete_only_functions
                        || value_type == luaur_vm::enums::lua_type::lua_Type::LUA_TFUNCTION as i32;

                    if !key.is_empty() && required_value_type && key.starts_with(prefix) {
                        let completed_component = &key[prefix.len()..];
                        let mut completion = edit_buffer.to_string();
                        completion.push_str(completed_component);

                        if value_type == luaur_vm::enums::lua_type::lua_Type::LUA_TFUNCTION as i32 {
                            completion.push('(');
                        }

                        add_completion_callback(&completion, &key);
                    }
                }

                luaur_vm::macros::lua_pop::lua_pop(l, 1);
            }

            if !crate::functions::try_replace_top_with_index::try_replace_top_with_index(l) {
                break;
            }
        }
    }
}
