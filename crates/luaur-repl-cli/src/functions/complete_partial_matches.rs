use alloc::string::String;
use core::ffi::CStr;

use luaur_common::functions::starts_with::startsWith;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::functions::lua_next::lua_next;
use luaur_vm::functions::lua_pushnil::lua_pushnil;
use luaur_vm::functions::lua_type::lua_type;
use luaur_vm::macros::lua_istable::lua_istable;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::macros::lua_tostring::lua_tostring;
use luaur_vm::type_aliases::lua_state::lua_State;

use crate::functions::try_replace_top_with_index::try_replace_top_with_index;

// Mirrors MaxTraversalLimit in Repl.cpp.
const MAX_TRAVERSAL_LIMIT: i32 = 50;

// completePartialMatches finds keys that match the specified 'prefix'
// Note: the table/object to be searched must be on the top of the Lua stack
pub unsafe fn complete_partial_matches(
    l: *mut lua_State,
    complete_only_functions: bool,
    edit_buffer: &str,
    prefix: &str,
    add_completion_callback: &dyn Fn(&str, &str),
) {
    let mut i = 0;
    while i < MAX_TRAVERSAL_LIMIT && lua_istable!(l, -1) {
        // table, key
        lua_pushnil(l);

        // Loop over all the keys in the current table
        while lua_next(l, -2) != 0 {
            if lua_type(l, -2) == lua_Type::LUA_TSTRING as i32 {
                // table, key, value
                let key_ptr = lua_tostring!(l, -2);
                let key = CStr::from_ptr(key_ptr).to_string_lossy();
                let value_type = lua_type(l, -1);

                // If the last separator was a ':' (i.e. a method call) then only functions should be completed.
                let required_value_type =
                    !complete_only_functions || value_type == lua_Type::LUA_TFUNCTION as i32;

                if !key.is_empty() && required_value_type && startsWith(&key, prefix) {
                    let completed_component = &key[prefix.len()..];
                    let mut completion = String::with_capacity(edit_buffer.len() + completed_component.len() + 1);
                    completion.push_str(edit_buffer);
                    completion.push_str(completed_component);
                    if value_type == lua_Type::LUA_TFUNCTION as i32 {
                        // Add an opening paren for function calls by default.
                        completion.push('(');
                    }
                    add_completion_callback(&completion, &key);
                }
            }
            lua_pop(l, 1);
        }

        // Replace the current table being searched with an __index table if one exists
        if !try_replace_top_with_index(l) {
            break;
        }

        i += 1;
    }
}
