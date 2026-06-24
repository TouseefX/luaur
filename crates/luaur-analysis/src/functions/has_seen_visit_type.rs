//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/VisitType.h:36:has_seen`
//! Source: `Analysis/include/Luau/VisitType.h:36-40` (hand-ported)
//!
//! C++ `bool hasSeen(std::unordered_set<void*>& seen, const void* tv)` — the
//! forgetting-set overload used by `TypeVisitor`. (An earlier translation
//! wrongly took `DenseHashSet`, silently giving TypeVisitor visit-once
//! semantics.)

#[allow(non_snake_case)]
pub fn has_seen(
    seen: &mut std::collections::HashSet<*mut core::ffi::c_void>,
    tv: *const core::ffi::c_void,
) -> bool {
    let ttv = tv as *mut core::ffi::c_void;
    !seen.insert(ttv)
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use has_seen as has_seen_unordered_set_void_void;
