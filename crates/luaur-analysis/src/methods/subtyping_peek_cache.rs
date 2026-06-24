use crate::records::subtyping::Subtyping;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::type_pair_hash::TypePairHash;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl Subtyping {
    pub fn peek_cache(&self) -> &DenseHashMap<(TypeId, TypeId), SubtypingResult, TypePairHash> {
        &self.result_cache
    }
}
