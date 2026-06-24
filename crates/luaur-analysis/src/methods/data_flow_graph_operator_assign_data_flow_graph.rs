use crate::records::data_flow_graph::DataFlowGraph;

impl DataFlowGraph {
    #[allow(non_snake_case)]
    pub fn operator_assign_mut(&mut self, other: DataFlowGraph) -> &mut Self {
        self.def_arena = other.def_arena;
        self.key_arena = other.key_arena;
        self.ast_defs = other.ast_defs;
        self.local_defs = other.local_defs;
        self.declared_defs = other.declared_defs;
        self.def_to_symbol = other.def_to_symbol;
        self.ast_refinement_keys = other.ast_refinement_keys;
        self
    }
}
