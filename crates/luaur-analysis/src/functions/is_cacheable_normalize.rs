//! Source: `Analysis/src/Normalize.cpp:875-894` (hand-ported)
//!
//! `static bool isCacheable(TypePackId tp, Set<TypeId>& seen)` — the type-pack
//! overload of `isCacheable`. The full body lives alongside its sibling
//! `TypeId` overload in `is_cacheable_normalize_alt_b`; this module re-exports
//! it under the cited node's name to preserve the one-item-per-file mapping.
pub use crate::functions::is_cacheable_normalize_alt_b::is_cacheable_type_pack_id_set_type_id as is_cacheable;
