//! Generated skeleton item.
//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/Module.h:29:scope_ptr`
//! Source: `Analysis/include/Luau/Module.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/Module.h
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Linter.h
//!   - includes -> source_file Analysis/include/Luau/FileResolver.h
//!   - includes -> source_file Ast/include/Luau/ParseOptions.h
//!   - includes -> source_file Ast/include/Luau/ParseResult.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeArena.h
//!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/Module.h
//!   - type_ref <- record Module (Analysis/include/Luau/Module.h)
//! - outgoing:
//!   - type_ref -> record Scope (Analysis/include/Luau/Scope.h)
//!   - translates_to -> rust_item ScopePtr

// C++ re-declares `using ScopePtr = std::shared_ptr<Scope>`; one canonical Rust alias.
pub use crate::type_aliases::scope_ptr_type::ScopePtr;
