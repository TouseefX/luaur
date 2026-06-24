//! Source: `Analysis/include/Luau/ControlFlowGraph.h:79-84` (hand-ported)
use crate::type_aliases::refinement_control_flow_graph::RefinementMember;
use crate::type_aliases::refinement_id_control_flow_graph::RefinementId;

/// C++ `template<typename T> const T* get(RefinementId r)` (ControlFlowGraph.h).
pub fn get_refinement_id_mut<T: RefinementMember>(r: RefinementId) -> *const T {
    unsafe {
        match T::get_if(&*r) {
            Some(x) => x as *const T,
            None => core::ptr::null(),
        }
    }
}
