extern crate alloc;

use alloc::vec::Vec;

use crate::records::scope::Scope;
use crate::records::stack_pusher_type_checker_2::StackPusher;

impl StackPusher {
    // C++ `explicit StackPusher(std::vector<NotNull<Scope>>& stack, Scope* scope)`
    // (TypeChecker2.cpp:66): pushes the scope onto the stack for the lifetime of
    // the guard.
    pub unsafe fn stack_pusher_vector_not_null_scope_scope(
        stack: *mut Vec<*mut Scope>,
        scope: *mut Scope,
    ) -> Self {
        StackPusher::new(stack, scope)
    }
}
