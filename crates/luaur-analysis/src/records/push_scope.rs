use crate::records::dfg_scope::DfgScope;
use crate::type_aliases::scope_stack::ScopeStack;

#[derive(Debug)]
pub struct PushScope {
    pub(crate) stack: *mut ScopeStack,
    pub(crate) previous_size: usize,
}

impl Drop for PushScope {
    fn drop(&mut self) {
        if self.previous_size == usize::MAX {
            return;
        }

        let stack = unsafe { &mut *self.stack };
        luaur_common::LUAU_ASSERT!(stack.len() > self.previous_size);
        while stack.len() > self.previous_size {
            stack.pop();
        }
        self.previous_size = usize::MAX;
    }
}
