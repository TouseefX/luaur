use crate::records::instantiation_2::Instantiation2;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_map::DenseHashMap;

pub fn instantiate_2(
    arena: *mut TypeArena,
    generic_substitutions: DenseHashMap<TypeId, TypeId>,
    generic_pack_substitutions: DenseHashMap<TypePackId, TypePackId>,
    subtyping: *mut Subtyping,
    scope: *mut Scope,
    tp: TypePackId,
) -> Option<TypePackId> {
    let mut instantiation = Instantiation2::instantiation_2_type_arena_dense_hash_map_type_id_type_id_dense_hash_map_type_pack_id_type_pack_id_not_null_subtyping_not_null_scope(
        arena,
        generic_substitutions,
        generic_pack_substitutions,
        subtyping,
        scope,
    );

    instantiation.clean_type_pack_id(tp).into()
}
