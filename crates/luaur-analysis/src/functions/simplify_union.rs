use crate::records::builtin_types::BuiltinTypes;
use crate::records::simplify_result::SimplifyResult;
use crate::records::type_arena::TypeArena;
use crate::records::type_simplifier::TypeSimplifier;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn simplify_union(
    builtin_types: *mut BuiltinTypes,
    arena: *mut TypeArena,
    left: TypeId,
    right: TypeId,
) -> SimplifyResult {
    let builtin_types = unsafe { builtin_types.as_ref() }.expect("builtin_types is null");
    let arena = unsafe { arena.as_ref() }.expect("arena is null");

    let mut s = TypeSimplifier {
        builtin_types,
        arena,
        blocked_types: DenseHashSet::new(core::ptr::null_mut()),
        recursion_depth: 0,
    };

    let res = s.union_(left, right);

    SimplifyResult {
        result: res,
        blocked_types: core::mem::replace(
            &mut s.blocked_types,
            DenseHashSet::new(core::ptr::null_mut()),
        ),
    }
}
