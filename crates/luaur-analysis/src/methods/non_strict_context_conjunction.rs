use crate::functions::simplify_intersection_simplify::simplify_intersection;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;
use alloc::collections::BTreeMap;

pub fn non_strict_context_conjunction(
    builtins: *mut BuiltinTypes,
    arena: *mut TypeArena,
    left: &NonStrictContext,
    right: &NonStrictContext,
) -> NonStrictContext {
    let mut conj = NonStrictContext {
        context: BTreeMap::new(),
    };

    for (&def, &left_ty) in &left.context {
        if let Some(right_ty) = right.find_def(def) {
            let result = simplify_intersection(builtins, arena, left_ty, right_ty);
            conj.context.insert(def, result.result);
        }
    }

    conj
}
