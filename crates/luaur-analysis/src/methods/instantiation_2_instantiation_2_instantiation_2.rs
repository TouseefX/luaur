use crate::records::instantiation_2::Instantiation2;
use crate::records::substitution::Substitution;
use crate::records::txn_log::TxnLog;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl Instantiation2 {
    pub fn instantiation_2_type_arena_dense_hash_map_type_id_type_id_dense_hash_map_type_pack_id_type_pack_id(
        arena: *mut TypeArena,
        generic_substitutions: DenseHashMap<TypeId, TypeId>,
        generic_pack_substitutions: DenseHashMap<TypePackId, TypePackId>,
    ) -> Self {
        Self::instantiation_2_type_arena_dense_hash_map_type_id_type_id_dense_hash_map_type_pack_id_type_pack_id_not_null_subtyping_not_null_scope(
            arena,
            generic_substitutions,
            generic_pack_substitutions,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        )
    }
}
