use crate::records::builtin_types::BuiltinTypes;
use crate::records::simplify_result::SimplifyResult;
use crate::records::type_arena::TypeArena;
use crate::records::type_ids::TypeIds;
use crate::records::type_simplifier::TypeSimplifier;

pub fn simplify_intersection_not_null_builtin_types_not_null_type_arena_type_ids(
    builtin_types: *mut BuiltinTypes,
    arena: *mut TypeArena,
    parts: TypeIds,
) -> SimplifyResult {
    let mut s = TypeSimplifier {
        builtin_types: builtin_types as *const BuiltinTypes,
        arena: arena as *const TypeArena,
        blocked_types: luaur_common::records::dense_hash_set::DenseHashSet::new(
            core::ptr::null_mut(),
        ),
        recursion_depth: 0,
    };

    let res = s.intersect_from_parts(parts);

    SimplifyResult {
        result: res,
        blocked_types: s.blocked_types,
    }
}
