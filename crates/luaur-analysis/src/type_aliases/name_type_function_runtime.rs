//! Generated skeleton item.
//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/TypeFunctionRuntime.h:212:name`
//! Source: `Analysis/include/Luau/TypeFunctionRuntime.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/TypeFunctionRuntime.h
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeFunctionError.h
//!   - includes -> source_file Analysis/include/Luau/TypeFunctionRuntimeBuilder.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Common/include/Luau/Variant.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/TypeFunctionRuntime.h
//!   - type_ref <- type_alias Name (Analysis/include/Luau/TypeFunctionRuntime.h)
//! - outgoing:
//!   - type_ref -> type_alias Name (Analysis/include/Luau/TypeFunctionRuntime.h)
//!   - translates_to -> rust_item Name

// Luau/TypeFwd.h:52 — using Name = std::string
pub type Name = alloc::string::String;
