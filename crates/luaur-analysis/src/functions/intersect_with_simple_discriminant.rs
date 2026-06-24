use crate::functions::is_simple_discriminant_simplify_alt_b::is_simple_discriminant_type_id;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::type_arena::TypeArena;
use crate::records::type_simplifier::TypeSimplifier;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn intersect_with_simple_discriminant(
    builtin_types: *mut BuiltinTypes,
    arena: *mut TypeArena,
    target: TypeId,
    discriminant: TypeId,
) -> Option<TypeId> {
    if !is_simple_discriminant_type_id(discriminant) {
        if is_simple_discriminant_type_id(target) {
            return intersect_with_simple_discriminant(builtin_types, arena, discriminant, target);
        }
        return None;
    }
    let mut s = TypeSimplifier {
        builtin_types: builtin_types as *const _,
        arena: arena as *const _,
        blocked_types: DenseHashSet::new(core::ptr::null_mut()),
        recursion_depth: 0,
    };
    s.intersect_with_simple_discriminant_type_id_type_id(target, discriminant)
}
