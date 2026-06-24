use crate::records::proposition_refinement::Proposition;
use crate::records::refinement_arena_refinement::RefinementArena;
use crate::records::refinement_key::RefinementKey;
use crate::records::typed_allocator::TypedAllocator;
use crate::type_aliases::refinement_id_refinement::RefinementId;
use crate::type_aliases::refinement_refinement::Refinement;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl RefinementArena {
    pub fn implicit_proposition_refinement_key_type_id(
        &mut self,
        key: *const RefinementKey,
        discriminant_ty: TypeId,
    ) -> RefinementId {
        if key.is_null() {
            return core::ptr::null_mut();
        }

        let refinement_ptr = self
            .allocator
            .allocate(Refinement::Proposition(Proposition {
                key,
                discriminantTy: discriminant_ty,
                implicitFromCall: true,
            }));

        LUAU_ASSERT!(!refinement_ptr.is_null());
        refinement_ptr
    }
}
