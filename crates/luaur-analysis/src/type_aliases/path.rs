//! Generated skeleton item.
//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/TypePath.h:235:path`
//! Source: `Analysis/include/Luau/TypePath.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/TypePath.h
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/TypeArena.h
//!   - includes -> source_file Analysis/include/Luau/TypeFwd.h
//!   - includes -> source_file Common/include/Luau/Variant.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/TypePath.h
//!   - type_ref <- record Path (Analysis/include/Luau/TypePath.h)
//! - outgoing:
//!   - type_ref -> record Path (Analysis/include/Luau/TypePath.h)
//!   - translates_to -> rust_item Path

// TypePath.h:235 — using Path = TypePath::Path (the record)
pub use crate::records::path::Path;
