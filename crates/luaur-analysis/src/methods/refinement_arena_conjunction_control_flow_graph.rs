//! Source: `Analysis/src/ControlFlowGraph.cpp:28-31` (hand-ported)
//! C++ `RefinementId RefinementArena::conjunction(RefinementId lhs, RefinementId rhs)`.
use crate::records::conjunction_control_flow_graph::Conjunction;
use crate::records::refinement_arena_control_flow_graph::RefinementArena;
use crate::type_aliases::refinement_control_flow_graph::Refinement;
use crate::type_aliases::refinement_id_control_flow_graph::RefinementId;

impl RefinementArena {
    pub fn conjunction_mut(&mut self, lhs: RefinementId, rhs: RefinementId) -> RefinementId {
        // C++: return NotNull{allocator.allocate(Conjunction{lhs, rhs})};
        self.allocator
            .allocate(Refinement::Conjunction(Conjunction { lhs, rhs }))
    }
}
