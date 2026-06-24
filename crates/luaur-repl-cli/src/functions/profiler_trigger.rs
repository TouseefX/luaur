use alloc::collections::BTreeMap;
use alloc::string::String;
use core::ffi::CStr;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread::JoinHandle;

use luaur_vm::functions::lua_callbacks::lua_callbacks;
use luaur_vm::functions::lua_getinfo::lua_getinfo;
use luaur_vm::records::lua_callbacks::LuaCallbacks;
use luaur_vm::records::lua_debug::LuaDebug;
use luaur_vm::records::lua_state::lua_State;

// Trigger-side view of Profiler.cpp's file-static `gProfiler`. The sampling
// thread publishes `ticks` (atomic) and the VM-thread trigger consumes them,
// keeping `current_ticks`, the `stack_scratch` reuse buffer, the accumulated
// per-stack `data` and the per-GC-state timing array — exactly the fields
// `profilerTrigger` reads and writes. (Profiler.cpp uses a
// DenseHashMap<string,uint64> for `data`; a BTreeMap captures the same
// stack→ticks accumulation.)
pub(crate) struct ProfilerTriggerState {
    // static state (Profiler.cpp: callbacks / frequency / thread)
    pub(crate) callbacks: *mut LuaCallbacks,
    pub(crate) frequency: i32,
    pub(crate) thread: Option<JoinHandle<()>>,
    // loop<->trigger communication
    pub(crate) exit: AtomicBool,
    pub(crate) ticks: AtomicU64,
    pub(crate) samples: AtomicU64,
    // trigger-private + statistics
    pub(crate) current_ticks: u64,
    pub(crate) stack_scratch: String,
    pub(crate) data: Option<BTreeMap<String, u64>>,
    pub(crate) gc: [u64; 16],
}

pub(crate) static mut G_PROFILER: ProfilerTriggerState = ProfilerTriggerState {
    callbacks: core::ptr::null_mut(),
    frequency: 1000,
    thread: None,
    exit: AtomicBool::new(false),
    ticks: AtomicU64::new(0),
    samples: AtomicU64::new(0),
    current_ticks: 0,
    stack_scratch: String::new(),
    data: None,
    gc: [0; 16],
};

// Faithful port of Profiler.cpp's `static void profilerTrigger(lua_State* L, int gc)`.
pub unsafe fn profiler_trigger(l: *mut lua_State, gc: i32) {
    let profiler = core::ptr::addr_of_mut!(G_PROFILER).as_mut().unwrap();

    let current_ticks = profiler.ticks.load(Ordering::Relaxed);
    let elapsed_ticks = current_ticks - profiler.current_ticks;

    if elapsed_ticks != 0 {
        let stack = &mut profiler.stack_scratch;
        stack.clear();

        if gc > 0 {
            stack.push_str("GC,GC,");
        }

        let mut ar: LuaDebug = core::mem::zeroed();
        let mut level = 0;
        while lua_getinfo(l, level, c"sn".as_ptr(), &mut ar as *mut LuaDebug) != 0 {
            if !stack.is_empty() {
                stack.push(';');
            }

            if !ar.short_src.is_null() {
                stack.push_str(&CStr::from_ptr(ar.short_src).to_string_lossy());
            }
            stack.push(',');
            if !ar.name.is_null() {
                stack.push_str(&CStr::from_ptr(ar.name).to_string_lossy());
            }
            stack.push(',');
            if ar.linedefined > 0 {
                use core::fmt::Write;
                let _ = write!(stack, "{}", ar.linedefined);
            }

            level += 1;
        }

        if !stack.is_empty() {
            let key = stack.clone();
            let data = profiler.data.get_or_insert_with(BTreeMap::new);
            *data.entry(key).or_insert(0) += elapsed_ticks;
        }

        if gc > 0 {
            profiler.gc[gc as usize] += elapsed_ticks;
        }
    }

    profiler.current_ticks = current_ticks;

    if !profiler.callbacks.is_null() {
        (*profiler.callbacks).interrupt = None;
    } else {
        // The sampling-thread shim does not publish a callbacks pointer, so
        // clear the live state's interrupt directly — equivalent to C++'s
        // `gProfiler.callbacks->interrupt = nullptr`.
        (*lua_callbacks(l)).interrupt = None;
    }
}
