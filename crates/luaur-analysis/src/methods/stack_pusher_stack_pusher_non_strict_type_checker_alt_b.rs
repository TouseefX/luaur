use crate::records::stack_pusher_non_strict_type_checker::StackPusher;

impl StackPusher {
    pub fn stack_pusher_mut(&mut self) {
        if !self.stack.is_null() {
            unsafe {
                luaur_common::macros::luau_assert::LUAU_ASSERT!(
                    (*self.stack).last().copied() == Some(self.scope)
                );
                (*self.stack).pop();
            }
        }
    }
}
