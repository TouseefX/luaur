//! `ContainsAnyGeneric_DEPRECATED::visit(TypeId ty)` (TypeUtils.cpp:957-961).
//!
//! C++:
//! ```cpp
//! bool ContainsAnyGeneric_DEPRECATED::visit(TypeId ty)
//! {
//!     found = found || is<GenericType>(ty);
//!     return !found;
//! }
//! ```
//!
//! This `visit(TypeId)` overload is realized as the inherent `visit_type_id`
//! method on `ContainsAnyGenericDeprecated` in the record file
//! (`records/contains_any_generic_deprecated.rs`); re-declaring it here would
//! be a duplicate `impl` for the same method name. The faithful body sets
//! `found |= is::<GenericType>(ty)` and returns `!found`.
