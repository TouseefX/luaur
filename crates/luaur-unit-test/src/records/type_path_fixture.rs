use crate::records::fixture::Fixture;
use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
use luaur_analysis::records::type_arena::TypeArena;
use luaur_analysis::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_map::DenseHashMap;

pub struct TypePathFixture {
    pub base: Fixture,
    pub sff1: ScopedFastFlag,
    pub arena: TypeArena,
    pub empty_map_deprecated: DenseHashMap<TypePackId, TypePackId>,
}
