use alloc::string::String;
use luaur_vm::records::lua_state::lua_State;

use crate::functions::complete_indexer::complete_indexer;

pub fn get_completions(
    l: *mut lua_State,
    edit_buffer: &String,
    add_completion_callback: &mut dyn FnMut(&str, &str),
) {
    complete_indexer(l, edit_buffer, add_completion_callback);
}
