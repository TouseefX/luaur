use core::ffi::c_int;
use core::sync::atomic::{AtomicPtr, Ordering};
use luaur_vm::functions::lua_ref::lua_ref;
use luaur_vm::records::lua_state::lua_State;

pub(crate) struct Coverage {
    pub(crate) functions: alloc::vec::Vec<c_int>,
}

pub(crate) static G_COVERAGE: AtomicPtr<Coverage> = AtomicPtr::new(core::ptr::null_mut());

pub fn coverage_track(l: *mut lua_State, funcindex: c_int) {
    unsafe {
        let coverage_ptr = G_COVERAGE.load(Ordering::SeqCst);
        if coverage_ptr.is_null() {
            let mut coverage = Coverage {
                functions: alloc::vec::Vec::new(),
            };
            let r = lua_ref(l, funcindex);
            coverage.functions.push(r);
            G_COVERAGE.store(&mut coverage as *mut Coverage, Ordering::SeqCst);
        } else {
            let coverage = &mut *coverage_ptr;
            let r = lua_ref(l, funcindex);
            coverage.functions.push(r);
        }
    }
}
