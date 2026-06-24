use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use core::option::Option;

#[allow(non_snake_case)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeInstantiationCountMismatch {
    pub(crate) functionName: Option<String>,
    pub(crate) functionType: TypeId,
    pub(crate) providedTypes: usize,
    pub(crate) maximumTypes: usize,
    pub(crate) providedTypePacks: usize,
    pub(crate) maximumTypePacks: usize,
}

#[allow(non_snake_case)]
impl TypeInstantiationCountMismatch {
    pub fn functionName(&self) -> Option<&str> {
        self.functionName.as_deref()
    }

    pub fn functionType(&self) -> TypeId {
        self.functionType
    }

    pub fn providedTypes(&self) -> usize {
        self.providedTypes
    }

    pub fn maximumTypes(&self) -> usize {
        self.maximumTypes
    }

    pub fn providedTypePacks(&self) -> usize {
        self.providedTypePacks
    }

    pub fn maximumTypePacks(&self) -> usize {
        self.maximumTypePacks
    }
}
