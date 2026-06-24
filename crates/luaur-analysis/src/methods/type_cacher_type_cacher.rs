use crate::records::type_cacher::TypeCacher;
use crate::records::type_once_visitor::TypeOnceVisitor;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl TypeCacher {
    pub fn type_cacher(cached_types: *mut DenseHashSet<TypeId>) -> Self {
        Self {
            base: TypeOnceVisitor::new("TypeCacher".to_string(), true),
            cached_types,
            uncacheable: DenseHashSet::new(core::ptr::null()),
            uncacheable_packs: DenseHashSet::new(core::ptr::null()),
        }
    }
}
