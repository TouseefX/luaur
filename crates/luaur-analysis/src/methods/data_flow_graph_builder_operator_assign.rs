use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;

impl DataFlowGraphBuilder {
    pub fn operator_assign(&mut self, _other: &DataFlowGraphBuilder) -> &mut DataFlowGraphBuilder {
        unimplemented!("DataFlowGraphBuilder copy assignment is deleted in C++")
    }
}
