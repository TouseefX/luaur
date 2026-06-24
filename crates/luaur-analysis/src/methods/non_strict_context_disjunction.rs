use crate::functions::simplify_union::simplify_union;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::type_arena::TypeArena;

pub fn non_strict_context_disjunction(
    builtin_types: *mut BuiltinTypes,
    arena: *mut TypeArena,
    left: &NonStrictContext,
    right: &NonStrictContext,
) -> NonStrictContext {
    let mut disj = NonStrictContext {
        context: alloc::collections::BTreeMap::new(),
    };

    for (&def, &left_ty) in &left.context {
        if let Some(right_ty) = right.find_def(def) {
            let result = simplify_union(builtin_types, arena, left_ty, right_ty).result;
            disj.context.insert(def, result);
        } else {
            disj.context.insert(def, left_ty);
        }
    }

    for (&def, &right_ty) in &right.context {
        if left.find_def(def).is_none() {
            disj.context.insert(def, right_ty);
        }
    }

    disj
}
