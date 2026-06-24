use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExplicitFunctionAnnotationRecommended {
    pub(crate) recommended_args: Vec<(String, TypeId)>,
    pub(crate) recommended_return: TypeId,
}

#[allow(non_snake_case)]
impl ExplicitFunctionAnnotationRecommended {
    pub fn recommendedArgs(&self) -> &[(String, TypeId)] {
        &self.recommended_args
    }

    pub fn recommendedReturn(&self) -> TypeId {
        self.recommended_return
    }
}
