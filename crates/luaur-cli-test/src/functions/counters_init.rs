use luaur_vm::functions::lua_mainthread::lua_mainthread;
use luaur_vm::records::lua_state::lua_State;

pub(crate) struct Counters {
    pub(crate) L: *mut lua_State,
}

pub(crate) static mut gCounters: Counters = Counters {
    L: core::ptr::null_mut(),
};

pub fn counters_init(l: *mut lua_State) {
    unsafe {
        gCounters.L = lua_mainthread(l);
    }
}
