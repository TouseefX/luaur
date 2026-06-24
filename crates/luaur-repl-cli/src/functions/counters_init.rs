use alloc::vec::Vec;

use luaur_vm::functions::lua_mainthread::lua_mainthread;
use luaur_vm::type_aliases::lua_state::lua_State;

use crate::records::module_counters::ModuleCounters;

// Faithful port of Counters.cpp's file-static
//     struct Counters {
//         lua_State* L = nullptr;
//         std::vector<int> moduleRefs;
//         std::vector<ModuleCounters> moduleCounters;
//     } gCounters;
pub(crate) struct CountersState {
    pub(crate) l: *mut lua_State,
    pub(crate) module_refs: Vec<i32>,
    pub(crate) module_counters: Vec<ModuleCounters>,
}

pub(crate) static mut G_COUNTERS: CountersState = CountersState {
    l: core::ptr::null_mut(),
    module_refs: Vec::new(),
    module_counters: Vec::new(),
};

pub fn counters_init(l: *mut core::ffi::c_void) {
    unsafe {
        (*core::ptr::addr_of_mut!(G_COUNTERS)).l = lua_mainthread(l as *mut lua_State);
    }
}
