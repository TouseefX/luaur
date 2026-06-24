use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::type_pair_hash::TypePairHash;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_table::DenseHasher;

impl DenseHasher<(TypeId, TypeId)> for TypePairHash {
    fn hash(&self, x: &(TypeId, TypeId)) -> usize {
        self.operator_call(*x)
    }
}

// C++ `TypePairHash` provides `operator()` overloads for both `std::pair<TypeId,
// TypeId>` and `std::pair<TypePackId, TypePackId>` (used by `SeenTypePackSet`).
impl DenseHasher<(TypePackId, TypePackId)> for TypePairHash {
    fn hash(&self, x: &(TypePackId, TypePackId)) -> usize {
        self.operator_call_2(*x)
    }
}

impl Subtyping {
    pub fn cache(
        &mut self,
        _env: &mut SubtypingEnvironment,
        result: SubtypingResult,
        sub_ty: TypeId,
        super_ty: TypeId,
    ) -> SubtypingResult {
        let p = (sub_ty, super_ty);

        if result.is_cacheable {
            *self.result_cache.get_or_insert(p) = result.clone();
        }

        result
    }
}
