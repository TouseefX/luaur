//! Generated skeleton item.
//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/TypeInfer.h:32:scope_ptr`
//! Source: `Analysis/include/Luau/TypeInfer.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/TypeInfer.h
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Anyification.h
//!   - includes -> source_file Analysis/include/Luau/ControlFlow.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Instantiation.h
//!   - includes -> source_file Analysis/include/Luau/Module.h
//!   - includes -> source_file Analysis/include/Luau/Predicate.h
//!   - includes -> source_file Analysis/include/Luau/Substitution.h
//!   - includes -> source_file Analysis/include/Luau/Symbol.h
//!   - includes -> source_file Analysis/include/Luau/TxnLog.h
//!   - includes -> source_file Analysis/include/Luau/TypeFwd.h
//!   - includes -> source_file Analysis/include/Luau/TypeCheckLimits.h
//!   - includes -> source_file Analysis/include/Luau/TypeUtils.h
//!   - includes -> source_file Analysis/include/Luau/Unifier.h
//!   - includes -> source_file Analysis/include/Luau/UnifierSharedState.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/TypeInfer.h
//!   - type_ref <- record TypeChecker (Analysis/include/Luau/TypeInfer.h)
//! - outgoing:
//!   - type_ref -> record Scope (Analysis/include/Luau/Scope.h)
//!   - translates_to -> rust_item ScopePtr

// C++ re-declares `using ScopePtr = std::shared_ptr<Scope>`; one canonical Rust alias.
pub use crate::type_aliases::scope_ptr_type::ScopePtr;
