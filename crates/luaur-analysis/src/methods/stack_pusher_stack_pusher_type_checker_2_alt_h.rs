use crate::records::stack_pusher_type_checker_2::StackPusher;

impl StackPusher {
    // C++ move constructor `StackPusher(StackPusher&& other)` (TypeChecker2.cpp:85):
    // steals `other.stack` via `std::exchange(other.stack, nullptr)` so the
    // moved-from guard's destructor is a no-op. In Rust a value move already
    // leaves nothing behind to drop, so this is only used for an explicit
    // move-construct: null out the source's stack and take ownership.
    pub fn stack_pusher_stack_pusher_mut_2(mut other: StackPusher) -> Self {
        let stack = core::mem::replace(&mut other.stack, core::ptr::null_mut());
        let scope = other.scope;
        StackPusher { stack, scope }
    }
}
