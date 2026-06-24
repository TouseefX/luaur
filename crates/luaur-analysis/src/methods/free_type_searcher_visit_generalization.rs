use crate::records::free_type_searcher::FreeTypeSearcher;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl FreeTypeSearcher {
    /// C++ `bool FreeTypeSearcher::visit(TypeId ty)` (Generalization.cpp:91-98) —
    /// the generic dispatch override. Guards re-traversal via the cached-type
    /// set and the polarity-aware seen set; returns `true` so the base
    /// `traverse` descends into children for the variants the searcher does not
    /// specialize.
    pub fn visit_type_id(&mut self, ty: TypeId) -> bool {
        if unsafe { (*self.cached_types).contains(&ty) }
            || self.seen_with_current_polarity(ty as *const core::ffi::c_void)
        {
            return false;
        }

        LUAU_ASSERT!(!ty.is_null());
        true
    }
}
