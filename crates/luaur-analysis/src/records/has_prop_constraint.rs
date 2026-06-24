use crate::enums::value_context::ValueContext;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;

#[derive(Debug, Clone)]
pub struct HasPropConstraint {
    pub result_type: TypeId,
    pub subject_type: TypeId,
    pub prop: String,
    pub context: ValueContext,
    pub in_conditional: bool,
    pub suppress_simplification: bool,
}
