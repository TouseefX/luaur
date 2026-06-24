use crate::type_aliases::type_id::TypeId;
use alloc::collections::BTreeSet;
use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnknownPropButFoundLikeProp {
    pub(crate) table: TypeId,
    pub(crate) key: String,
    pub(crate) candidates: BTreeSet<String>,
}

#[allow(non_snake_case)]
impl UnknownPropButFoundLikeProp {
    pub fn table(&self) -> TypeId {
        self.table
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn candidates(&self) -> &BTreeSet<String> {
        &self.candidates
    }
}
