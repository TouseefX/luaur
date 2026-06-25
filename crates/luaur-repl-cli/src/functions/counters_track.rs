use luaur_vm::functions::lua_ref::lua_ref;
use luaur_vm::type_aliases::lua_state::lua_State;

use crate::functions::counters_init::G_COUNTERS;

// Faithful port of:
//     void countersTrack(lua_State* L, int funcindex) {
//         int ref = lua_ref(L, funcindex);
//         gCounters.moduleRefs.push_back(ref);
//     }
pub fn counters_track(l: *mut lua_State, funcindex: i32) {
    unsafe {
        let ref_id = lua_ref(l, funcindex);
        (*core::ptr::addr_of_mut!(G_COUNTERS))
            .module_refs
            .push(ref_id);
    }
}
