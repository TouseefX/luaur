use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct PropertyTypes {
    pub(crate) types_of_prop: Vec<TypeId>,
    pub(crate) missing_prop: Vec<TypeId>,
}

#[allow(non_snake_case)]
impl PropertyTypes {
    pub fn foundOneProp(&self) -> bool {
        !self.types_of_prop.is_empty()
    }

    pub fn noneMissingProp(&self) -> bool {
        self.missing_prop.is_empty()
    }

    pub fn foundMissingProp(&self) -> bool {
        !self.missing_prop.is_empty()
    }
}
