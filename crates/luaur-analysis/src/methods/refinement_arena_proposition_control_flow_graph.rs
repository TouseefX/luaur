use crate::records::proposition_control_flow_graph::Proposition;
use crate::records::refinement_arena_control_flow_graph::RefinementArena;
use crate::type_aliases::def_id_control_flow_graph::DefId;
use crate::type_aliases::refinement_control_flow_graph::Refinement;
use crate::type_aliases::refinement_id_control_flow_graph::RefinementId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl RefinementArena {
    pub fn proposition_def_id_bool(&mut self, def: DefId, sense: bool) -> RefinementId {
        let refinement_ptr = self
            .allocator
            .allocate(Refinement::Proposition(Proposition {
                ptr: def,
                r#type: None,
                is_typeof: false,
                sense,
            }));

        LUAU_ASSERT!(!refinement_ptr.is_null());
        refinement_ptr
    }
}
