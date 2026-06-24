use core::sync::atomic::{AtomicBool, AtomicI32, AtomicPtr, Ordering};
use luaur_vm::records::lua_state::lua_State;

#[derive(Debug)]
pub struct ConformanceDebuggerState {
    pub breakhits: AtomicI32,
    pub interruptedthread: AtomicPtr<lua_State>,
    pub singlestep: AtomicBool,
    pub stephits: AtomicI32,
}

impl ConformanceDebuggerState {
    pub const fn new() -> Self {
        Self {
            breakhits: AtomicI32::new(0),
            interruptedthread: AtomicPtr::new(core::ptr::null_mut()),
            singlestep: AtomicBool::new(false),
            stephits: AtomicI32::new(0),
        }
    }

    pub fn reset(&self, singlestep: bool) {
        self.breakhits.store(0, Ordering::SeqCst);
        self.interruptedthread
            .store(core::ptr::null_mut(), Ordering::SeqCst);
        self.singlestep.store(singlestep, Ordering::SeqCst);
        self.stephits.store(0, Ordering::SeqCst);
    }
}

pub static CONFORMANCE_DEBUGGER_STATE: ConformanceDebuggerState = ConformanceDebuggerState::new();
