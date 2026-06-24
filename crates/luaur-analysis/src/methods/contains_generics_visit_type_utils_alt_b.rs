//! `ContainsGenerics::visit(TypeId ty, const GenericType&)` (TypeUtils.cpp:1000-1004).
//!
//! C++:
//! ```cpp
//! bool visit(TypeId ty, const GenericType&) override
//! {
//!     found |= generics->contains(ty);
//!     return true;
//! }
//! ```
//!
//! Realized faithfully as the inherent `visit_type_id_generic_type` method on
//! `ContainsGenerics` in the record file (`records/contains_generics.rs`),
//! whose body does `self.found |= (*self.generics).contains(key)` and returns
//! `true`. Re-declaring it here would be a duplicate `impl` for the same
//! method name.
