use luaur_vm::functions::lua_mainthread::lua_mainthread;
use luaur_vm::records::lua_state::lua_State;

pub fn coverage_init(l: *mut lua_State) {
    unsafe {
        crate::functions::coverage_active::gCoverage.L =
            lua_mainthread(l) as *mut core::ffi::c_void;
    }
}
