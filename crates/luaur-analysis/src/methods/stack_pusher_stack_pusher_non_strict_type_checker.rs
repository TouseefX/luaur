use crate::records::scope::Scope;
use crate::records::stack_pusher_non_strict_type_checker::StackPusher;
use alloc::vec::Vec;

impl StackPusher {
    pub fn stack_pusher_vector_not_null_scope_scope_mut(
        &mut self,
        stack: *mut Vec<*mut Scope>,
        scope: *mut Scope,
    ) -> Self {
        unsafe {
            (*stack).push(scope);
        }
        StackPusher { stack, scope }
    }
}
