use crate::enums::polarity::Polarity;
use crate::records::generalization_params::GeneralizationParams;
use crate::records::scope::Scope;
use crate::records::type_visitor::TypeVisitor;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::records::insertion_ordered_map::InsertionOrderedMap;

#[derive(Debug, Clone)]
pub struct FreeTypeSearcher {
    pub base: TypeVisitor,
    pub scope: *mut Scope,
    pub cached_types: *mut DenseHashSet<TypeId>,
    pub is_within_function: bool,
    pub polarity: Polarity,
    pub seen_positive: DenseHashSet<*const core::ffi::c_void>,
    pub seen_negative: DenseHashSet<*const core::ffi::c_void>,
    pub negative_types: DenseHashMap<*const core::ffi::c_void, usize>,
    pub positive_types: DenseHashMap<*const core::ffi::c_void, usize>,
    pub types: InsertionOrderedMap<TypeId, GeneralizationParams>,
    pub type_packs: InsertionOrderedMap<TypePackId, GeneralizationParams>,
    pub unsealed_tables: DenseHashSet<TypeId>,
}
