use luaur_vm::type_aliases::lua_state::lua_State;

pub unsafe fn get_completions(
    l: *mut lua_State,
    edit_buffer: &str,
    add_completion_callback: &dyn Fn(&str, &str),
) {
    crate::functions::complete_indexer::complete_indexer(l, edit_buffer, add_completion_callback);
}
