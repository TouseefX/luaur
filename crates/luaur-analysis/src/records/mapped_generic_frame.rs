use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct MappedGenericFrame {
    pub(crate) mappings: DenseHashMap<TypePackId, Option<TypePackId>>,
    pub(crate) parent_scope_index: Option<usize>,
    pub(crate) children: DenseHashSet<usize>,
}
