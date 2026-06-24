use crate::records::data_flow_graph::DataFlowGraph;

impl DataFlowGraph {
    /// In C++, this method is deleted to prevent copying.
    /// In Rust, `DataFlowGraph` does not implement `Clone` or `Copy`,
    /// so an explicit assignment operator is not provided.
    #[allow(dead_code)]
    pub fn operator_assign(&mut self, _other: &DataFlowGraph) -> &mut DataFlowGraph {
        panic!("DataFlowGraph copy assignment is deleted in C++");
    }
}
