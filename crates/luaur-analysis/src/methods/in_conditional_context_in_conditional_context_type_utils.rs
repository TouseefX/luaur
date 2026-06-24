use crate::enums::type_context::TypeContext;
use crate::records::in_conditional_context::InConditionalContext;

impl InConditionalContext {
    pub fn new(c: *mut TypeContext, new_value: TypeContext) -> Self {
        let old_value = unsafe { *c };
        unsafe {
            *c = new_value;
        }
        Self {
            type_context: c,
            old_value,
        }
    }
}

#[allow(non_snake_case)]
pub fn in_conditional_context_in_conditional_context(
    c: *mut TypeContext,
    new_value: TypeContext,
) -> InConditionalContext {
    InConditionalContext::new(c, new_value)
}
