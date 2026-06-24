//! Source: `Analysis/include/Luau/Subtyping.h` (hand-ported; fields only)

use crate::records::generic_bounds::GenericBounds;
use crate::records::mapped_generic_environment::MappedGenericEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::type_pair_hash::TypePairHash;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug)]
pub struct SubtypingEnvironment {
    pub parent: *mut SubtypingEnvironment,
    pub mapped_generics: DenseHashMap<TypeId, Vec<GenericBounds>>,
    pub mapped_generic_packs: MappedGenericEnvironment,
    pub substitutions: DenseHashMap<TypeId, TypeId>,
    pub seen_set_cache: DenseHashMap<(TypeId, TypeId), SubtypingResult, TypePairHash>,
    pub iteration_count: i32,
}
