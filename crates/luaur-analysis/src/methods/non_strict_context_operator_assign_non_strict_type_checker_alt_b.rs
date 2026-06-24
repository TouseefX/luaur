use crate::records::non_strict_context::NonStrictContext;

impl NonStrictContext {
    #[allow(non_snake_case)]
    pub fn operator_assign_mut(&mut self, other: NonStrictContext) -> &mut Self {
        *self = other;
        self
    }
}
