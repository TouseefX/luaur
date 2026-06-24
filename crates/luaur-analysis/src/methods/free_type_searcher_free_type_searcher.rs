use crate::enums::polarity::Polarity;
use crate::records::free_type_searcher::FreeTypeSearcher;
use crate::records::scope::Scope;
use crate::records::type_visitor::TypeVisitor;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::records::insertion_ordered_map::InsertionOrderedMap;

impl FreeTypeSearcher {
    pub fn free_type_searcher(scope: *mut Scope, cached_types: *mut DenseHashSet<TypeId>) -> Self {
        Self {
            base: TypeVisitor::type_visitor("FreeTypeSearcher".to_string(), true),
            scope,
            cached_types,
            is_within_function: false,
            polarity: Polarity::Positive,
            seen_positive: DenseHashSet::new(core::ptr::null()),
            seen_negative: DenseHashSet::new(core::ptr::null()),
            negative_types: DenseHashMap::new(core::ptr::null()),
            positive_types: DenseHashMap::new(core::ptr::null()),
            types: InsertionOrderedMap::new(),
            type_packs: InsertionOrderedMap::new(),
            unsealed_tables: DenseHashSet::new(core::ptr::null()),
        }
    }
}
