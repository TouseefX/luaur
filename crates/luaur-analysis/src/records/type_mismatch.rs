use crate::records::type_error::TypeError;
use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeMismatch {
    pub wanted_type: TypeId,
    pub given_type: TypeId,
    pub reason: alloc::string::String,
    pub error: Option<alloc::sync::Arc<TypeError>>,
    pub context: crate::enums::context_error::Context,
}
