//! Generated skeleton item.
//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/Linter.h:20:scope_ptr`
//! Source: `Analysis/include/Luau/Linter.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/Linter.h
//! - source_includes:
//!   - includes -> source_file Config/include/Luau/LinterConfig.h
//!   - includes -> source_file Ast/include/Luau/Location.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/Linter.h
//! - outgoing:
//!   - type_ref -> record Scope (Analysis/include/Luau/Scope.h)
//!   - translates_to -> rust_item ScopePtr

// C++ re-declares `using ScopePtr = std::shared_ptr<Scope>`; one canonical Rust alias.
pub use crate::type_aliases::scope_ptr_type::ScopePtr;
