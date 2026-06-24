pub fn setup_state(l: *mut luaur_vm::type_aliases::lua_state::lua_State) {
    unsafe {
        luaur_vm::functions::lua_l_openlibs::lua_l_openlibs(l);
        luaur_vm::functions::lua_l_sandbox::lua_l_sandbox(l);
    }
}
