use crate::records::refinement_key::RefinementKey;
use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Proposition {
    pub(crate) key: *const RefinementKey,
    pub(crate) discriminantTy: TypeId,
    pub(crate) implicitFromCall: bool,
}

#[allow(non_snake_case)]
impl Proposition {
    pub fn key(&self) -> *const RefinementKey {
        self.key
    }

    pub fn discriminantTy(&self) -> TypeId {
        self.discriminantTy
    }

    pub fn implicitFromCall(&self) -> bool {
        self.implicitFromCall
    }
}
