use crate::records::tarjan::Tarjan;
use crate::records::txn_log::TxnLog;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct Substitution {
    pub(crate) base: Tarjan,
    pub(crate) arena: *mut TypeArena,
    pub(crate) new_types: DenseHashMap<TypeId, TypeId>,
    pub(crate) new_packs: DenseHashMap<TypePackId, TypePackId>,
    pub(crate) replaced_types: DenseHashSet<TypeId>,
    pub(crate) replaced_type_packs: DenseHashSet<TypePackId>,
    pub(crate) no_traverse_types: DenseHashSet<TypeId>,
    pub(crate) no_traverse_type_packs: DenseHashSet<TypePackId>,
}
