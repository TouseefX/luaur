//! Source: `Analysis/src/NonStrictTypeChecker.cpp:60-64` (hand-ported)
use crate::records::stack_pusher_non_strict_type_checker::StackPusher;

impl StackPusher {
    /// C++ `StackPusher(StackPusher&& other)` move-ctor (steals `other.stack` via
    /// `std::exchange`). Rust transfers ownership on move, so this special member
    /// has no call site.
    pub fn stack_pusher_stack_pusher_mut(&mut self, _other: StackPusher) -> Self {
        unreachable!("C++ StackPusher move-ctor; Rust moves by value — no call site")
    }
}
