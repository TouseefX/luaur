//! Source: `Analysis/include/Luau/ConstraintGenerator.h:49-61` (hand-ported)
use crate::type_aliases::refinement_id_refinement::RefinementId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct InferencePack {
    pub tp: TypePackId,
    pub refinements: Vec<RefinementId>,
}

impl InferencePack {
    // C++ `InferencePack() = default;` with `TypePackId tp = nullptr;`.
    pub fn inference_pack() -> Self {
        InferencePack {
            tp: core::ptr::null(),
            refinements: Vec::new(),
        }
    }

    // C++ `explicit InferencePack(TypePackId tp, const std::vector<RefinementId>& refinements = {})`.
    pub fn inference_pack_type_pack_id_vector_refinement_id(
        tp: TypePackId,
        refinements: Vec<RefinementId>,
    ) -> Self {
        InferencePack { tp, refinements }
    }
}
