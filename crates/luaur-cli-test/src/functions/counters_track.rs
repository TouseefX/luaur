use core::ffi::c_int;
use core::sync::atomic::{AtomicPtr, Ordering};
use luaur_common::records::small_vector::SmallVector;
use luaur_vm::functions::lua_ref::lua_ref;
use luaur_vm::records::lua_state::lua_State;

pub(crate) struct Counters {
    pub(crate) module_refs: SmallVector<c_int, 8>,
}

pub(crate) static G_COUNTERS: AtomicPtr<Counters> = AtomicPtr::new(core::ptr::null_mut());

pub fn counters_track(l: *mut lua_State, funcindex: c_int) {
    unsafe {
        let counters_ptr = G_COUNTERS.load(Ordering::SeqCst);
        if counters_ptr.is_null() {
            let mut counters = Counters {
                module_refs: SmallVector::new(),
            };
            let r = lua_ref(l, funcindex);
            counters.module_refs.push(r);
            G_COUNTERS.store(&mut counters as *mut Counters, Ordering::SeqCst);
        } else {
            let counters = &mut *counters_ptr;
            let r = lua_ref(l, funcindex);
            counters.module_refs.push(r);
        }
    }
}
