use alloc::boxed::Box;
use luaur_common::LUAU_ASSERT;

#[allow(non_camel_case_types)]
pub struct ScopedExit {
    pub(crate) func: Option<Box<dyn FnOnce()>>,
}

impl ScopedExit {
    #[allow(non_snake_case)]
    pub fn new(f: Box<dyn FnOnce()>) -> Self {
        LUAU_ASSERT!(true); // In C++, LUAU_ASSERT(func) checks if the std::function is non-null.
        Self { func: Some(f) }
    }
}

impl Default for ScopedExit {
    fn default() -> Self {
        Self { func: None }
    }
}

impl Drop for ScopedExit {
    fn drop(&mut self) {
        if let Some(f) = self.func.take() {
            f();
        }
    }
}

// ScopedExit in C++ is used for RAII-style cleanup.
// The C++ implementation uses std::function<void()>, which is roughly Box<dyn FnOnce()>.
// We use Option to allow moving the function out during drop or swap (move semantics).

#[allow(non_snake_case)]
impl ScopedExit {
    /// Moves the content of `other` into `self`, similar to C++ move assignment.
    pub fn move_from(&mut self, other: &mut Self) {
        self.func = other.func.take();
    }
}

unsafe impl Send for ScopedExit {}
unsafe impl Sync for ScopedExit {}
