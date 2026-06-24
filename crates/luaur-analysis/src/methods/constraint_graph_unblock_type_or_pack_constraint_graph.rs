use crate::functions::follow_type::follow_type_id;
use crate::records::constraint_graph::ConstraintGraph;
use crate::type_aliases::constraint_vertex::ConstraintVertex;
use crate::type_aliases::type_id::TypeId;

impl ConstraintGraph {
    pub fn unblock_type_or_pack_type_id(&mut self, vertex: TypeId) {
        self.repair_type_references_type_id(vertex);
        let followed = unsafe { follow_type_id(vertex) };
        self.clear_reverse_dependencies_of(ConstraintVertex::V0(followed));
    }
}
