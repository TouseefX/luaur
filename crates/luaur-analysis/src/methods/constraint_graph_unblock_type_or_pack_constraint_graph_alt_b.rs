use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::records::constraint_graph::ConstraintGraph;
use crate::type_aliases::constraint_vertex::ConstraintVertex;
use crate::type_aliases::type_pack_id::TypePackId;

impl ConstraintGraph {
    pub fn unblock_type_or_pack_type_pack_id(&mut self, vertex: TypePackId) {
        self.repair_type_references_type_pack_id(vertex);
        let vertex = unsafe { follow_type_pack_id(vertex) };
        let _vertex = vertex;

        self.clear_reverse_dependencies_of(ConstraintVertex::V2(core::ptr::null()));
    }
}
