use core::sync::atomic::{AtomicBool, AtomicI64, AtomicUsize, Ordering};
use std::thread::JoinHandle;

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

pub fn profiler_stop() {
    unsafe {
        extern "Rust" {
            static mut gProfiler: ProfilerState;
        }

        let profiler = core::ptr::addr_of_mut!(gProfiler).as_mut().unwrap();
        profiler.exit.store(true, Ordering::Relaxed);

        let thread = core::mem::replace(&mut profiler.thread, None);
        if let Some(handle) = thread {
            let _ = handle.join();
        }
    }
}
