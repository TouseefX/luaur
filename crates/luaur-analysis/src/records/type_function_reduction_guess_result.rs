use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeFunctionReductionGuessResult {
    pub(crate) guessed_function_annotations: Vec<(String, TypeId)>,
    pub(crate) guessed_return_type: TypeId,
    pub(crate) should_recommend_annotation: bool,
}
