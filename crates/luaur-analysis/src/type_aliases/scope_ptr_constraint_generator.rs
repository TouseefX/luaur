//! Generated skeleton item.
//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/ConstraintGenerator.h:30:scope_ptr`
//! Source: `Analysis/include/Luau/ConstraintGenerator.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/ConstraintGenerator.h
//! - source_includes:
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/Constraint.h
//!   - includes -> source_file Analysis/include/Luau/ConstraintGraph.h
//!   - includes -> source_file Analysis/include/Luau/ConstraintSet.h
//!   - includes -> source_file Analysis/include/Luau/ControlFlow.h
//!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
//!   - includes -> source_file Common/include/Luau/HashUtil.h
//!   - includes -> source_file Common/include/Luau/InsertionOrderedMap.h
//!   - includes -> source_file Analysis/include/Luau/Module.h
//!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
//!   - includes -> source_file Analysis/include/Luau/Polarity.h
//!   - includes -> source_file Analysis/include/Luau/Refinement.h
//!   - includes -> source_file Analysis/include/Luau/Set.h
//!   - includes -> source_file Analysis/include/Luau/Symbol.h
//!   - includes -> source_file Analysis/include/Luau/TypeFwd.h
//!   - includes -> source_file Analysis/include/Luau/TypeIds.h
//!   - includes -> source_file Analysis/include/Luau/TypeUtils.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/ConstraintGenerator.h
//!   - type_ref <- record ConstraintGenerator (Analysis/include/Luau/ConstraintGenerator.h)
//!   - type_ref <- record FunctionSignature (Analysis/include/Luau/ConstraintGenerator.h)
//! - outgoing:
//!   - type_ref -> record Scope (Analysis/include/Luau/Scope.h)
//!   - translates_to -> rust_item ScopePtr

// C++ re-declares `using ScopePtr = std::shared_ptr<Scope>`; one canonical Rust alias.
pub use crate::type_aliases::scope_ptr_type::ScopePtr;
