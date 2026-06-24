extern crate alloc;

use alloc::string::String;

use core::sync::atomic::{AtomicBool, AtomicU64};

use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug)]
pub struct Profiler {
    pub(crate) callbacks: *mut core::ffi::c_void,
    pub(crate) frequency: i32,
    pub(crate) thread: (),
    pub(crate) exit: AtomicBool,
    pub(crate) ticks: AtomicU64,
    pub(crate) samples: AtomicU64,
    pub(crate) current_ticks: u64,
    pub(crate) stack_scratch: String,
    pub(crate) data: DenseHashMap<String, u64>,
    pub(crate) gc: [u64; 16],
}

#[allow(dead_code)]
static mut gProfiler: Option<Profiler> = None;
