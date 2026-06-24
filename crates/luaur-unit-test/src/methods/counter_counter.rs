use crate::records::counter::Counter;

#[allow(non_upper_case_globals)]
static mut instanceCount: i32 = 0;

impl Counter {
    /// C++: `Counter() { ++instanceCount; id = instanceCount; }`
    /// (tests/Parser.test.cpp:41). An associated constructor — each call bumps
    /// the static instance count and records it as this instance's `id`.
    pub fn counter_counter() -> Self {
        unsafe {
            instanceCount += 1;
            Counter { id: instanceCount }
        }
    }

    /// C++ tests reset the static counter directly via `Counter::instanceCount = 0;`
    /// (tests/Parser.test.cpp:98). Rust has no assignable associated static, so the
    /// reset is exposed as a method.
    pub fn reset_instance_count() {
        unsafe {
            instanceCount = 0;
        }
    }
}
