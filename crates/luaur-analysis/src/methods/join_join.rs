use crate::records::join::Join;
use crate::type_aliases::def_id_control_flow_graph::DefId;

impl Join {
    pub fn join(definition: DefId) -> Self {
        Self {
            definition,
            operands: alloc::vec::Vec::new(),
        }
    }
}
