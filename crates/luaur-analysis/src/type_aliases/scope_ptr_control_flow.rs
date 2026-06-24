//! Generated skeleton item.
//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/ControlFlow.h:10:scope_ptr`
//! Source: `Analysis/include/Luau/ControlFlow.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/ControlFlow.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/ControlFlow.h
//! - outgoing:
//!   - type_ref -> record Scope (Analysis/include/Luau/Scope.h)
//!   - translates_to -> rust_item ScopePtr

// C++ re-declares `using ScopePtr = std::shared_ptr<Scope>`; one canonical Rust alias.
pub use crate::type_aliases::scope_ptr_type::ScopePtr;
