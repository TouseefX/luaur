use core::sync::atomic::{AtomicBool, AtomicI64, AtomicUsize, Ordering};
use luaur_vm::records::lua_state::lua_State;
use std::thread::{self, JoinHandle};

#[allow(non_snake_case)]
struct ProfilerCallbacks {
    interrupt: fn(),
}

#[allow(non_snake_case)]
struct ProfilerState {
    exit: AtomicBool,
    frequency: i64,
    ticks: AtomicI64,
    samples: AtomicUsize,
    callbacks: *mut ProfilerCallbacks,
    thread: Option<JoinHandle<()>>,
}

extern "C" {
    // In Luau, lua_callbacks is a macro or an internal accessor that returns the callback block from the state.
    // Based on the C++ source `gProfiler.callbacks = lua_callbacks(L);`, it returns the pointer to the callbacks.
    fn lua_callbacks(L: *mut lua_State) -> *mut ProfilerCallbacks;
}

extern "Rust" {
    static mut gProfiler: ProfilerState;
    fn profiler_loop();
}

pub fn profiler_start(l: *mut lua_State, frequency: i32) {
    unsafe {
        let profiler = core::ptr::addr_of_mut!(gProfiler).as_mut().unwrap();

        profiler.frequency = frequency as i64;
        profiler.callbacks = lua_callbacks(l);
        profiler.exit.store(false, Ordering::Relaxed);
        profiler.thread = Some(thread::spawn(|| {
            profiler_loop();
        }));
    }
}
