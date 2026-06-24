use luaur_vm::functions::lua_clock::lua_clock;

use core::sync::atomic::{AtomicBool, AtomicI64, AtomicUsize, Ordering};
use std::thread;

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
}

// These globals mirror the C++ `gProfiler` layout.
// They are provided elsewhere in the crate; this file only needs to update them.
extern "Rust" {
    static mut gProfiler: ProfilerState;
    fn profilerTrigger();
}

pub fn profiler_loop() {
    let mut last = lua_clock();

    loop {
        // Safety: `gProfiler` is a shared global managed by the profiler subsystem.
        let should_exit = unsafe {
            (*core::ptr::addr_of_mut!(gProfiler).as_ref().unwrap())
                .exit
                .load(Ordering::Relaxed)
        };
        if should_exit {
            break;
        }

        let now = lua_clock();

        // Safety: `frequency` is treated as an immutable configuration value.
        let frequency = unsafe { core::ptr::addr_of!(gProfiler).read().frequency } as f64;

        if now - last >= 1.0 / frequency {
            let ticks = ((now - last) * 1e6) as i64;

            unsafe {
                let profiler = core::ptr::addr_of_mut!(gProfiler).as_mut().unwrap();

                profiler.ticks.fetch_add(ticks, Ordering::Relaxed);
                profiler.samples.fetch_add(1, Ordering::Relaxed);

                if !profiler.callbacks.is_null() {
                    (*profiler.callbacks).interrupt = profiler_trigger_wrapper;
                }
            }

            last += ticks as f64 * 1e-6;
        } else {
            thread::yield_now();
        }
    }
}

fn profiler_trigger_wrapper() {
    // `profilerTrigger` may be declared as `unsafe` in the linked translation unit.
    // The C++ code treats this as a plain function callback, so we bridge to a safe fn pointer.
    unsafe {
        profilerTrigger();
    }
}
