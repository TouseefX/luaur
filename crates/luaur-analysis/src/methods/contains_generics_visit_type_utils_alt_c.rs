//! `ContainsGenerics::visit(TypeId ty, const TypeFunctionInstanceType&)` (TypeUtils.cpp:1006-1009).
//!
//! C++:
//! ```cpp
//! bool visit(TypeId ty, const TypeFunctionInstanceType&) override
//! {
//!     return !found;
//! }
//! ```
//!
//! Realized faithfully as the inherent
//! `visit_type_id_type_function_instance_type` method on `ContainsGenerics` in
//! the record file (`records/contains_generics.rs`), whose body is
//! `!self.found`. Re-declaring it here would be a duplicate `impl` for the same
//! method name.
