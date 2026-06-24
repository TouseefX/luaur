use alloc::string::String;
use core::ffi::{c_char, c_int};

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
        // The required context shows these functions as pub fn name(); stubs.
        // We must cast them to the correct signature to call them.
        let lua_checkstack_ptr: unsafe extern "C" fn(*mut lua_State, c_int) -> c_int =
            core::mem::transmute(lua_checkstack as *const ());
        let lua_pushvalue_ptr: unsafe extern "C" fn(*mut lua_State, c_int) =
            core::mem::transmute(lua_pushvalue as *const ());
        let lua_pushlstring_ptr: unsafe extern "C" fn(*mut lua_State, *const c_char, usize) =
            core::mem::transmute(lua_pushlstring as *const ());
        let lua_remove_ptr: unsafe extern "C" fn(*mut lua_State, c_int) =
            core::mem::transmute(lua_remove as *const ());
        let lua_type_ptr: unsafe extern "C" fn(*mut lua_State, c_int) -> c_int =
            core::mem::transmute(lua_type as *const ());

        lua_checkstack_ptr(l, LUA_MINSTACK);

        // Push the global variable table to begin the search
        lua_pushvalue_ptr(l, LUA_GLOBALSINDEX);

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
            lua_pushlstring_ptr(l, prefix.as_ptr() as *const c_char, prefix.len());
            safe_get_table(l, -2);
            lua_remove_ptr(l, -2);

            // Manual expansion of lua_istable! using the transmuted pointer to avoid stub signature mismatch
            let is_table =
                lua_type_ptr(l, -1) == (luaur_vm::enums::lua_type::lua_Type::LUA_TTABLE as i32);

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
