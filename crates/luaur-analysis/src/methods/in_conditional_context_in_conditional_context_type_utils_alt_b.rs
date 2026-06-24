use crate::records::in_conditional_context::InConditionalContext;

impl Drop for InConditionalContext {
    fn drop(&mut self) {
        unsafe {
            *self.type_context = self.old_value.clone();
        }
    }
}

#[allow(non_snake_case)]
pub fn in_conditional_context_in_conditional_context() {
    // Kept as a no-op to match the scheduled method symbol name; actual logic is in Drop.
}
