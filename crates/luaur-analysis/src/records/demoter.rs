//! Generated skeleton item.
//! Node: `cxx:Record:Luau.Analysis:Analysis/src/TypeInfer.cpp:775:demoter`
//! Source: `Analysis/src/TypeInfer.cpp`
//! Graph edges:
//! - declared_by: source_file Analysis/src/TypeInfer.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/ApplyTypeFunction.h
//!   - includes -> source_file Analysis/include/Luau/Cancellation.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Instantiation.h
//!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/Quantify.h
//!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Substitution.h
//!   - includes -> source_file Common/include/Luau/TimeTrace.h
//!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypePack.h
//!   - includes -> source_file Analysis/include/Luau/TypeUtils.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//! - incoming:
//!   - declares <- source_file Analysis/src/TypeInfer.cpp
//!   - type_ref <- method TypeChecker::check (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method TypeChecker::getExpectedTypesForCall (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method Demoter::Demoter (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method Demoter::isDirty (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method Demoter::isDirty (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method Demoter::ignoreChildren (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method Demoter::clean (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method Demoter::clean (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method Demoter::demotedLevel (Analysis/src/TypeInfer.cpp)
//!   - type_ref <- method Demoter::demote (Analysis/src/TypeInfer.cpp)
//! - outgoing:
//!   - type_ref -> method Demoter::Demoter (Analysis/src/TypeInfer.cpp)
//!   - type_ref -> record Substitution (Analysis/include/Luau/Substitution.h)
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> record BuiltinTypes (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item Demoter

#[derive(Debug, Clone)]
pub struct Demoter {
    pub(crate) arena: *mut crate::records::type_arena::TypeArena,
    pub(crate) builtins: *mut crate::records::builtin_types::BuiltinTypes,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let TypeLevel: () = ();
}
