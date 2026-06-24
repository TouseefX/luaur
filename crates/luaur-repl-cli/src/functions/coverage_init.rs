use alloc::vec::Vec;

use luaur_vm::functions::lua_mainthread::lua_mainthread;
use luaur_vm::type_aliases::lua_state::lua_State;

// Faithful port of Repl.cpp's file-static `struct Coverage { lua_State* L;
// std::vector<int> functions; } gCoverage;`. The REPL drives this on a single
// thread, mirroring the C++ static.
pub(crate) struct CoverageState {
    pub(crate) l: *mut lua_State,
    pub(crate) functions: Vec<i32>,
}

pub(crate) static mut G_COVERAGE: CoverageState = CoverageState {
    l: core::ptr::null_mut(),
    functions: Vec::new(),
};

pub fn coverage_init(l: *mut lua_State) {
    unsafe {
        (*core::ptr::addr_of_mut!(G_COVERAGE)).l = lua_mainthread(l);
    }
}
