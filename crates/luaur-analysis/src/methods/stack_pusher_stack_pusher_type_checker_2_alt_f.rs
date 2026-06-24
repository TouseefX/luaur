//! Faithful port of `StackPusher::~StackPusher` (TypeChecker2.cpp:73-80).
//!
//! The C++ destructor asserts the guard's scope is on top of the stack and pops
//! it. In Rust this RAII behavior is realized by the `Drop for StackPusher`
//! implementation on the record (`records::stack_pusher_type_checker_2`), which
//! performs exactly that assert-and-pop. This free-function node is the
//! destructor's translation slot and has no callers of its own; the cleanup it
//! describes lives entirely in the `Drop` impl, so the body is empty.
pub fn stack_pusher_stack_pusher() {}
