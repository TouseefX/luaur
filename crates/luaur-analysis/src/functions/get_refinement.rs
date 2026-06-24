//! Source: `Analysis/include/Luau/Refinement.h:59-63` (hand-ported)
use crate::type_aliases::refinement_id_refinement::RefinementId;
use crate::type_aliases::refinement_refinement::RefinementMember;

/// C++ `template<typename T> const T* get(RefinementId refinement)`.
pub fn get_refinement_id<T: RefinementMember>(refinement: RefinementId) -> *const T {
    unsafe {
        match T::get_if(&*refinement) {
            Some(r) => r as *const T,
            None => core::ptr::null(),
        }
    }
}
