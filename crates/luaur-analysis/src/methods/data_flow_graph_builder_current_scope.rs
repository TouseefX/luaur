use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::dfg_scope::DfgScope;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl DataFlowGraphBuilder {
    pub fn current_scope(&mut self) -> *mut DfgScope {
        LUAU_ASSERT!(!self.scope_stack.is_empty());
        *self.scope_stack.last().unwrap()
    }
}
