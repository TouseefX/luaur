use crate::records::non_strict_context::NonStrictContext;

impl NonStrictContext {
    pub fn non_strict_context_non_strict_context_mut(&mut self, other: NonStrictContext) -> Self {
        self.non_strict_context_non_strict_context(&other)
    }
}
