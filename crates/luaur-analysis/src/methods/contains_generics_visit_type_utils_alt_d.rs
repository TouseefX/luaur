//! `ContainsGenerics::visit(TypePackId tp, const GenericTypePack&)` (TypeUtils.cpp:1011-1015).
//!
//! C++:
//! ```cpp
//! bool visit(TypePackId tp, const GenericTypePack&) override
//! {
//!     found |= generics->contains(tp);
//!     return !found;
//! }
//! ```
//!
//! Realized faithfully as the inherent `visit_type_pack_id_generic_type_pack`
//! method on `ContainsGenerics` in the record file
//! (`records/contains_generics.rs`), whose body does
//! `self.found |= (*self.generics).contains(key)` and returns `!self.found`.
//! Re-declaring it here would be a duplicate `impl` for the same method name.
