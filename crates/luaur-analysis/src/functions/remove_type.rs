use crate::records::builtin_types::BuiltinTypes;
use crate::records::type_arena::TypeArena;
use crate::records::type_remover::TypeRemover;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn remove_type(
    arena: *mut TypeArena,
    builtin_types: *mut BuiltinTypes,
    haystack: TypeId,
    needle: TypeId,
) {
    let mut tr = TypeRemover {
        builtin_types,
        arena,
        needle,
        seen: DenseHashSet::new(core::ptr::null()),
    };
    tr.process(haystack);
}
