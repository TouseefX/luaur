use crate::records::refine::Refine;
use crate::type_aliases::def_id_control_flow_graph::DefId;
use crate::type_aliases::refinement_control_flow_graph::Refinement;

impl Refine {
    pub fn refine(definition: DefId, prop: *const Refinement) -> Self {
        Self { definition, prop }
    }
}
