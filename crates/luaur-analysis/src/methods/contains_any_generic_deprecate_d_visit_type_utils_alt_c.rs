//! `ContainsAnyGeneric_DEPRECATED::visit(TypePackId ty)` (TypeUtils.cpp:963-967).
//!
//! C++:
//! ```cpp
//! bool ContainsAnyGeneric_DEPRECATED::visit(TypePackId ty)
//! {
//!     found = found || is<GenericTypePack>(follow(ty));
//!     return !found;
//! }
//! ```
//!
//! This `visit(TypePackId)` overload is realized as the inherent
//! `visit_type_pack_id` method on `ContainsAnyGenericDeprecated` in the record
//! file (`records/contains_any_generic_deprecated.rs`); re-declaring it here
//! would be a duplicate `impl` for the same method name. The faithful body sets
//! `found |= is::<GenericTypePack>(follow(ty))` and returns `!found`.
