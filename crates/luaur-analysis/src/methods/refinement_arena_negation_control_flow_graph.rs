//! Source: `Analysis/src/ControlFlowGraph.cpp:38-49` (hand-ported)
//! C++ `RefinementId RefinementArena::negation(RefinementId r)`.
use crate::records::conjunction_control_flow_graph::Conjunction;
use crate::records::disjunction_control_flow_graph::Disjunction;
use crate::records::negation_control_flow_graph::Negation;
use crate::records::proposition_control_flow_graph::Proposition;
use crate::records::refinement_arena_control_flow_graph::RefinementArena;
use crate::type_aliases::refinement_control_flow_graph::{Refinement, RefinementMember};
use crate::type_aliases::refinement_id_control_flow_graph::RefinementId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl RefinementArena {
    pub fn negation_mut(&mut self, r: RefinementId) -> RefinementId {
        // C++ `get<T>(r)` == `get_if<T>(r.get())`. `r` is `*mut Refinement`.
        let refinement: &Refinement = unsafe { &*r };

        // if (auto* conj = get<Conjunction>(r))
        //     return disjunction(negation(conj->lhs), negation(conj->rhs));
        if let Some(conj) = <Conjunction as RefinementMember>::get_if(refinement) {
            let (lhs, rhs) = (conj.lhs, conj.rhs);
            let nl = self.negation_mut(lhs);
            let nr = self.negation_mut(rhs);
            return self.disjunction_mut(nl, nr);
        }

        // if (auto* disj = get<Disjunction>(r))
        //     return conjunction(negation(disj->lhs), negation(disj->rhs));
        if let Some(disj) = <Disjunction as RefinementMember>::get_if(refinement) {
            let (lhs, rhs) = (disj.lhs, disj.rhs);
            let nl = self.negation_mut(lhs);
            let nr = self.negation_mut(rhs);
            return self.conjunction_mut(nl, nr);
        }

        // if (auto* neg = get<Negation>(r))
        //     return neg->refinement;
        if let Some(neg) = <Negation as RefinementMember>::get_if(refinement) {
            return neg.refinement;
        }

        // LUAU_ASSERT(get<Proposition>(r));
        LUAU_ASSERT!(<Proposition as RefinementMember>::get_if(refinement).is_some());
        // return NotNull{allocator.allocate(Negation{r})};
        self.allocator
            .allocate(Refinement::Negation(Negation { refinement: r }))
    }
}
