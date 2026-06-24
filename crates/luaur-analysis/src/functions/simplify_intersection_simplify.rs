use crate::records::builtin_types::BuiltinTypes;
use crate::records::simplify_result::SimplifyResult;
use crate::records::type_arena::TypeArena;
use crate::records::type_simplifier::TypeSimplifier;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn simplify_intersection(
    builtin_types: *mut BuiltinTypes,
    arena: *mut TypeArena,
    left: TypeId,
    right: TypeId,
) -> SimplifyResult {
    let mut s = TypeSimplifier {
        builtin_types: builtin_types as *const BuiltinTypes,
        arena: arena as *const TypeArena,
        blocked_types: DenseHashSet::new(core::ptr::null_mut()),
        recursion_depth: 0,
    };

    let res = s.intersect(left, right);

    SimplifyResult {
        result: res,
        blocked_types: s.blocked_types,
    }
}
