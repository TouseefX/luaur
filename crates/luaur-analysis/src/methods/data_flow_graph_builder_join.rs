use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::dfg_scope::DfgScope;

impl DataFlowGraphBuilder {
    pub fn join(&mut self, p: *mut DfgScope, a: *mut DfgScope, b: *mut DfgScope) {
        self.join_bindings(p, unsafe { &*a }, unsafe { &*b });
        self.join_props(p, unsafe { &*a }, unsafe { &*b });
    }
}
