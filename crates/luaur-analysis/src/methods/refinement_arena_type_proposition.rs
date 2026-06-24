use crate::records::proposition_control_flow_graph::Proposition;
use crate::records::refinement_arena_control_flow_graph::RefinementArena;
use crate::type_aliases::def_id_control_flow_graph::DefId;
use crate::type_aliases::refinement_control_flow_graph::Refinement;
use crate::type_aliases::refinement_id_control_flow_graph::RefinementId;
use alloc::string::String;

pub fn refinement_arena_type_proposition(
    arena: &mut RefinementArena,
    def: DefId,
    r#type: Option<String>,
    is_typeof: bool,
    sense: bool,
) -> RefinementId {
    arena
        .allocator
        .allocate(Refinement::Proposition(Proposition {
            ptr: def,
            r#type,
            is_typeof,
            sense,
        }))
}
