//! @interface-stub
use crate::type_aliases::refinement_id_refinement::RefinementId;
use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct Inference {
    pub ty: TypeId,
    pub refinement: RefinementId,
}

impl luaur_common::records::dense_hash_table::DenseDefault for Inference {
    fn dense_default() -> Self {
        Self {
            ty: core::ptr::null(),
            refinement: core::ptr::null_mut(),
        }
    }
}

impl Inference {
    // C++: `Inference()` — `ty` default-initialized to nullptr, no refinement.
    // (Analysis/include/Luau/ConstraintGenerator.h)
    pub fn inference() -> Self {
        Self {
            ty: core::ptr::null(),
            refinement: core::ptr::null_mut(),
        }
    }
    // C++: `Inference(TypeId ty, RefinementId refinement = nullptr)`.
    pub fn inference_type_id_refinement_id(ty: TypeId, refinement: RefinementId) -> Self {
        Self { ty, refinement }
    }
}
