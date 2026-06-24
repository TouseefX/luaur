//! Generated skeleton item.
//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/Scope.h:22:scope_ptr`
//! Source: `Analysis/include/Luau/Scope.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/Scope.h
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Def.h
//!   - includes -> source_file Analysis/include/Luau/LValue.h
//!   - includes -> source_file Ast/include/Luau/Location.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/Symbol.h
//!   - includes -> source_file Analysis/include/Luau/Unifiable.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/Scope.h
//!   - type_ref <- record Scope (Analysis/include/Luau/Scope.h)
//! - outgoing:
//!   - type_ref -> record Scope (Analysis/include/Luau/Scope.h)
//!   - translates_to -> rust_item ScopePtr

// C++ re-declares `using ScopePtr = std::shared_ptr<Scope>`; one canonical Rust alias.
pub use crate::type_aliases::scope_ptr_type::ScopePtr;
