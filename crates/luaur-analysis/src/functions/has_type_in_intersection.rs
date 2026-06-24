//! Source: `Analysis/include/Luau/Type.h:1266-1276` (hand-ported)
use crate::type_aliases::type_id::TypeId;

/// C++ `template<typename T> bool hasTypeInIntersection(TypeId ty)` — tests
/// whether the (flattened) intersection of `ty` contains a `get<T>` member. The
/// Rust port resolves variant membership per-type via `match`/get-if traits, so
/// callers are monomorphized; this unbounded template generic has no call site.
pub fn has_type_in_intersection<T>(_ty: TypeId) -> bool {
    unreachable!("C++ `hasTypeInIntersection<T>` template generic; Rust callers monomorphize the variant predicate — no call site")
}
