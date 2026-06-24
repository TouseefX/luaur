//! `ContainsGenerics::visit(TypeId ty)` (TypeUtils.cpp:995-998).
//!
//! C++:
//! ```cpp
//! bool visit(TypeId ty) override
//! {
//!     return !found;
//! }
//! ```
//!
//! Realized faithfully as the inherent `visit_type_id` method on
//! `ContainsGenerics` in the record file (`records/contains_generics.rs`),
//! whose body is `!self.found`. Re-declaring it here would be a duplicate
//! `impl` for the same method name.
