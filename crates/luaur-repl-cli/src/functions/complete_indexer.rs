use core::ffi::c_char;

use luaur_vm::functions::lua_checkstack::lua_checkstack;
use luaur_vm::functions::lua_pushlstring::lua_pushlstring;
use luaur_vm::functions::lua_pushvalue::lua_pushvalue;
use luaur_vm::functions::lua_remove::lua_remove;
use luaur_vm::macros::lua_globalsindex::LUA_GLOBALSINDEX;
use luaur_vm::macros::lua_istable::lua_istable;
use luaur_vm::macros::lua_minstack::LUA_MINSTACK;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::type_aliases::lua_state::lua_State;

use crate::functions::complete_partial_matches::complete_partial_matches;
use crate::functions::safe_get_table::safe_get_table;
use crate::functions::try_replace_top_with_index::try_replace_top_with_index;

pub unsafe fn complete_indexer(
    l: *mut lua_State,
    edit_buffer: &str,
    add_completion_callback: &dyn Fn(&str, &str),
) {
    let mut lookup = edit_buffer;
    let mut complete_only_functions = false;

    lua_checkstack(l, LUA_MINSTACK);

    // Push the global variable table to begin the search
    lua_pushvalue(l, LUA_GLOBALSINDEX);

    loop {
        let sep = lookup.find(['.', ':']);
        let prefix = match sep {
            Some(s) => &lookup[..s],
            None => lookup,
        };

        match sep {
            None => {
                complete_partial_matches(
                    l,
                    complete_only_functions,
                    edit_buffer,
                    prefix,
                    add_completion_callback,
                );
                break;
            }
            Some(sep) => {
                // find the key in the table
                lua_pushlstring(l, prefix.as_ptr() as *const c_char, prefix.len());
                safe_get_table(l, -2);
                lua_remove(l, -2);

                if lua_istable!(l, -1) || try_replace_top_with_index(l) {
                    complete_only_functions = lookup.as_bytes()[sep] == b':';
                    lookup = &lookup[sep + 1..];
                } else {
                    // Unable to search for keys, so stop searching
                    break;
                }
            }
        }
    }

    lua_pop(l, 1);
}
