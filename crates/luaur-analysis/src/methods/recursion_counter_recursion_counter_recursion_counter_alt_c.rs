use crate::records::recursion_counter::RecursionCounter;

impl RecursionCounter {
    /// C++ `RecursionCounter::RecursionCounter(int* count) : count(count) { ++(*count); }`
    /// (`Analysis/src/RecursionCounter.cpp:14`).
    pub fn recursion_counter_i32(count: *mut core::ffi::c_int) -> RecursionCounter {
        unsafe {
            *count += 1;
        }
        RecursionCounter { count }
    }
}
