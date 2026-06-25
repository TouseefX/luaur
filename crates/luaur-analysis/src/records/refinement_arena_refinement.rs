//! Source: `Analysis/src/Refinement.cpp` (hand-ported)
use crate::records::conjunction_refinement::Conjunction;
use crate::records::disjunction_refinement::Disjunction;
use crate::records::equivalence::Equivalence;
use crate::records::negation_refinement::Negation;
use crate::records::proposition_refinement::Proposition;
use crate::records::refinement_key::RefinementKey;
use crate::records::typed_allocator::TypedAllocator;
use crate::records::variadic::Variadic;
use crate::type_aliases::refinement_id_refinement::RefinementId;
use crate::type_aliases::refinement_refinement::Refinement;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct RefinementArena {
    pub(crate) allocator: TypedAllocator<Refinement>,
}

impl RefinementArena {
    // Analysis/src/Refinement.cpp:8 — RefinementId RefinementArena::variadic(const std::vector<RefinementId>& refis)
    pub fn variadic(&mut self, refis: &[RefinementId]) -> RefinementId {
        // bool hasRefinements = false;
        // for (RefinementId r : refis)
        //     hasRefinements |= bool(r);
        let mut has_refinements = false;
        for r in refis {
            has_refinements |= !r.is_null();
        }

        // if (!hasRefinements)
        //     return nullptr;
        if !has_refinements {
            return core::ptr::null_mut();
        }

        // return NotNull{allocator.allocate(Variadic{refis})};
        self.allocator.allocate(Refinement::Variadic(Variadic {
            refinements: refis.to_vec(),
        }))
    }

    // Analysis/src/Refinement.cpp:20 — RefinementId RefinementArena::negation(RefinementId refinement)
    pub fn negation(&mut self, refinement: RefinementId) -> RefinementId {
        // if (!refinement)
        //     return nullptr;
        if refinement.is_null() {
            return core::ptr::null_mut();
        }

        // return NotNull{allocator.allocate(Negation{refinement})};
        self.allocator
            .allocate(Refinement::Negation(Negation { refinement }))
    }

    // Analysis/src/Refinement.cpp:28 — RefinementId RefinementArena::conjunction(RefinementId lhs, RefinementId rhs)
    pub fn conjunction(&mut self, lhs: RefinementId, rhs: RefinementId) -> RefinementId {
        // if (!lhs && !rhs)
        //     return nullptr;
        if lhs.is_null() && rhs.is_null() {
            return core::ptr::null_mut();
        }

        // return NotNull{allocator.allocate(Conjunction{lhs, rhs})};
        self.allocator
            .allocate(Refinement::Conjunction(Conjunction { lhs, rhs }))
    }

    // Analysis/src/Refinement.cpp:36 — RefinementId RefinementArena::disjunction(RefinementId lhs, RefinementId rhs)
    pub fn disjunction(&mut self, lhs: RefinementId, rhs: RefinementId) -> RefinementId {
        // if (!lhs && !rhs)
        //     return nullptr;
        if lhs.is_null() && rhs.is_null() {
            return core::ptr::null_mut();
        }

        // return NotNull{allocator.allocate(Disjunction{lhs, rhs})};
        self.allocator
            .allocate(Refinement::Disjunction(Disjunction { lhs, rhs }))
    }

    // Analysis/src/Refinement.cpp:44 — RefinementId RefinementArena::equivalence(RefinementId lhs, RefinementId rhs)
    pub fn equivalence(&mut self, lhs: RefinementId, rhs: RefinementId) -> RefinementId {
        // if (!lhs && !rhs)
        //     return nullptr;
        if lhs.is_null() && rhs.is_null() {
            return core::ptr::null_mut();
        }

        // return NotNull{allocator.allocate(Equivalence{lhs, rhs})};
        self.allocator
            .allocate(Refinement::Equivalence(Equivalence { lhs, rhs }))
    }

    // Analysis/src/Refinement.cpp:52 — RefinementId RefinementArena::proposition(const RefinementKey* key, TypeId discriminantTy)
    pub fn proposition(
        &mut self,
        key: *const RefinementKey,
        discriminant_ty: TypeId,
    ) -> RefinementId {
        // if (!key)
        //     return nullptr;
        if key.is_null() {
            return core::ptr::null_mut();
        }

        // return NotNull{allocator.allocate(Proposition{key, discriminantTy, false})};
        self.allocator
            .allocate(Refinement::Proposition(Proposition {
                key,
                discriminantTy: discriminant_ty,
                implicitFromCall: false,
            }))
    }

    // Analysis/src/Refinement.cpp:60 — RefinementId RefinementArena::implicitProposition(const RefinementKey* key, TypeId discriminantTy)
    pub fn implicit_proposition(
        &mut self,
        key: *const RefinementKey,
        discriminant_ty: TypeId,
    ) -> RefinementId {
        // if (!key)
        //     return nullptr;
        if key.is_null() {
            return core::ptr::null_mut();
        }

        // return NotNull{allocator.allocate(Proposition{key, discriminantTy, true})};
        self.allocator
            .allocate(Refinement::Proposition(Proposition {
                key,
                discriminantTy: discriminant_ty,
                implicitFromCall: true,
            }))
    }
}
