use crate::records::type_once_visitor::TypeOnceVisitor;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct TypeCacher {
    pub base: TypeOnceVisitor,
    pub cached_types: *mut DenseHashSet<TypeId>,
    pub uncacheable: DenseHashSet<TypeId>,
    pub uncacheable_packs: DenseHashSet<TypePackId>,
}
