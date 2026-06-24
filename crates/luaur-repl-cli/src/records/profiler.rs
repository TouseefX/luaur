use alloc::string::String;
use core::sync::atomic::{AtomicBool, AtomicU64};
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug)]
pub struct Profiler {
    pub(crate) callbacks: *mut core::ffi::c_void,
    pub(crate) frequency: i32,
    pub(crate) thread: std::thread::JoinHandle<()>,
    pub(crate) exit: AtomicBool,
    pub(crate) ticks: AtomicU64,
    pub(crate) samples: AtomicU64,
    pub(crate) current_ticks: u64,
    pub(crate) stack_scratch: String,
    pub(crate) data: DenseHashMap<String, u64>,
    pub(crate) gc: [u64; 16],
}

impl Default for Profiler {
    fn default() -> Self {
        Self {
            callbacks: core::ptr::null_mut(),
            frequency: 1000,
            thread: std::thread::spawn(|| {}),
            exit: AtomicBool::new(false),
            ticks: AtomicU64::new(0),
            samples: AtomicU64::new(0),
            current_ticks: 0,
            stack_scratch: String::new(),
            data: DenseHashMap::new(String::new()),
            gc: [0; 16],
        }
    }
}
