use crate::records::builtin_types::BuiltinTypes;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug)]
pub struct TypeRemover {
    pub(crate) builtin_types: *mut BuiltinTypes,
    pub(crate) arena: *mut TypeArena,
    pub(crate) needle: TypeId,
    pub(crate) seen: DenseHashSet<TypeId>,
}
