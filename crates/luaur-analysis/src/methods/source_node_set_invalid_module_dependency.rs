//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Frontend.h:64:source_node_set_invalid_module_dependency`
//! Source: `Analysis/include/Luau/Frontend.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/Frontend.h
//! - source_includes:
//!   - includes -> source_file Config/include/Luau/Config.h
//!   - includes -> source_file Analysis/include/Luau/GlobalTypes.h
//!   - includes -> source_file Analysis/include/Luau/Module.h
//!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
//!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Set.h
//!   - includes -> source_file Analysis/include/Luau/TypeCheckLimits.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/Frontend.h
//!   - calls <- method Frontend::recordItemResult (Analysis/src/Frontend.cpp)
//! - outgoing:
//!   - type_ref -> record SourceNode (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item SourceNode::setInvalidModuleDependency

// Dead duplicate skeleton node: the canonical method is implemented elsewhere.
pub fn source_node_set_invalid_module_dependency() {
    unreachable!("canonical SourceNode::set_invalid_module_dependency lives in crates/luau-analysis/src/records/source_node.rs; this skeleton node is unused");
}
