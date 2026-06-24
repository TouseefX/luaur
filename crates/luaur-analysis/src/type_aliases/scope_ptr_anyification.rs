//! Generated skeleton item.
//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/Anyification.h:17:scope_ptr`
//! Source: `Analysis/include/Luau/Anyification.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/Anyification.h
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Substitution.h
//!   - includes -> source_file Analysis/include/Luau/TypeFwd.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/Anyification.h
//!   - type_ref <- record Anyification (Analysis/include/Luau/Anyification.h)
//! - outgoing:
//!   - type_ref -> record Scope (Analysis/include/Luau/Scope.h)
//!   - translates_to -> rust_item ScopePtr

// C++ re-declares `using ScopePtr = std::shared_ptr<Scope>`; one canonical Rust alias.
pub use crate::type_aliases::scope_ptr_type::ScopePtr;
