use alloc::string::String;
use core::ffi::c_char;

use crate::functions::complete_partial_matches::complete_partial_matches;
use crate::functions::safe_get_table::safe_get_table;
use crate::functions::try_replace_top_with_index::try_replace_top_with_index;

use luaur_vm::functions::lua_checkstack::lua_checkstack;
use luaur_vm::functions::lua_pushlstring::lua_pushlstring;
use luaur_vm::functions::lua_pushvalue::lua_pushvalue;
use luaur_vm::functions::lua_remove::lua_remove;
use luaur_vm::functions::lua_type::lua_type;
use luaur_vm::macros::lua_globalsindex::LUA_GLOBALSINDEX;
use luaur_vm::macros::lua_minstack::LUA_MINSTACK;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::records::lua_state::lua_State;

pub fn complete_indexer(
    l: *mut lua_State,
    edit_buffer: &String,
    add_completion_callback: &mut dyn FnMut(&str, &str),
) {
    let mut lookup: &str = edit_buffer.as_str();
    let mut complete_only_functions = false;

    unsafe {
        lua_checkstack(l, LUA_MINSTACK);

        // Push the global variable table to begin the search
        lua_pushvalue(l, LUA_GLOBALSINDEX);

        loop {
            let bytes = lookup.as_bytes();
            let mut sep: Option<usize> = None;

            for (i, &b) in bytes.iter().enumerate() {
                if b == b'.' || b == b':' {
                    sep = Some(i);
                    break;
                }
            }

            let sep_idx = match sep {
                Some(i) => i,
                None => {
                    complete_partial_matches(
                        l,
                        complete_only_functions,
                        edit_buffer,
                        lookup,
                        add_completion_callback,
                    );
                    break;
                }
            };

            let prefix = &lookup[..sep_idx];

            // find the key in the table
            lua_pushlstring(l, prefix.as_ptr() as *const c_char, prefix.len());
            safe_get_table(l, -2);
            lua_remove(l, -2);

            let is_table =
                lua_type(l, -1) == (luaur_vm::enums::lua_type::lua_Type::LUA_TTABLE as i32);

            if is_table || try_replace_top_with_index(l) {
                complete_only_functions = lookup.as_bytes()[sep_idx] == b':';
                lookup = &lookup[sep_idx + 1..];
            } else {
                // Unable to search for keys, so stop searching
                break;
            }
        }

        lua_pop(l, 1);
    }
}
