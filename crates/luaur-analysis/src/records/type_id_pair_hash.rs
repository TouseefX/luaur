use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_table::{DenseEq, DenseHasher};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct TypeIdPairHash {
    pub(crate) _unused: Option<core::convert::Infallible>,
}

unsafe impl Send for TypeIdPairHash {}
unsafe impl Sync for TypeIdPairHash {}

impl TypeIdPairHash {
    /// C++ `size_t hashOne(TypeId key) const` (UnifierSharedState.h:16).
    fn hash_one(key: TypeId) -> usize {
        let k = key as usize;
        (k >> 4) ^ (k >> 9)
    }
}

impl DenseHasher<(TypeId, TypeId)> for TypeIdPairHash {
    /// C++ `size_t operator()(const std::pair<TypeId, TypeId>& x) const`.
    fn hash(&self, key: &(TypeId, TypeId)) -> usize {
        Self::hash_one(key.0) ^ (Self::hash_one(key.1) << 1)
    }
}

impl DenseEq<(TypeId, TypeId)> for TypeIdPairHash {
    fn eq(&self, a: &(TypeId, TypeId), b: &(TypeId, TypeId)) -> bool {
        a == b
    }
}
