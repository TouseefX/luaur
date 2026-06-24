use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use alloc::vec::Vec;
use std::sync::{Condvar, Mutex};

/// A `std::function<void()>` task. `None` models an empty `std::function` — the
/// falsy sentinel the C++ pushes (`push({})`) to terminate a worker's loop.
pub type Task = Option<Box<dyn FnOnce() + Send + 'static>>;

/// Port of `struct TaskScheduler` (`CLI/src/Analyze.cpp:323-392`).
///
/// A thread pool scheduler for parallel task execution, mirroring the C++ design:
/// `std::mutex` + `std::condition_variable` guarding a `std::queue<std::function<void()>>`,
/// with one `std::thread` per worker. Native-only; not portable to wasm32-unknown-unknown.
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct TaskScheduler {
    pub(crate) thread_count: u32,
    pub(crate) workers: Vec<std::thread::JoinHandle<()>>,
    pub(crate) tasks: Arc<TaskQueue>,
}

/// The shared `mtx` / `cv` / `tasks` triple from the C++ `TaskScheduler`.
pub struct TaskQueue {
    pub(crate) mtx: Mutex<VecDeque<Task>>,
    pub(crate) cv: Condvar,
}

impl core::fmt::Debug for TaskQueue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TaskQueue").finish_non_exhaustive()
    }
}

impl TaskQueue {
    pub(crate) fn new() -> Self {
        Self {
            mtx: Mutex::new(VecDeque::new()),
            cv: Condvar::new(),
        }
    }
}

impl TaskScheduler {
    /// `static unsigned getThreadCount()` (`CLI/src/Analyze.cpp:375-378`):
    /// `return std::max(std::thread::hardware_concurrency(), 1u);`
    pub fn get_thread_count() -> u32 {
        core::cmp::max(
            std::thread::available_parallelism()
                .map(|n| n.get() as u32)
                .unwrap_or(1),
            1,
        )
    }
}

impl Drop for TaskScheduler {
    /// `~TaskScheduler()` (`CLI/src/Analyze.cpp:339-346`):
    /// pushes one empty task per worker, then joins them.
    fn drop(&mut self) {
        crate::methods::task_scheduler_task_scheduler_analyze_alt_b::task_scheduler_destructor(self);
    }
}
