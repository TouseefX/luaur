use crate::records::substitution::Substitution;
use crate::records::type_function_context::TypeFunctionContext;
use crate::type_aliases::type_id::TypeId;
use core::ptr::NonNull;

#[derive(Debug, Clone)]
pub struct RefineTypeScrubber {
    pub(crate) base: Substitution,
    pub(crate) ctx: NonNull<TypeFunctionContext>,
    pub(crate) needle: TypeId,
}
