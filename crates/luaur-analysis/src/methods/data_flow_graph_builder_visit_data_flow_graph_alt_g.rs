use crate::enums::control_flow::ControlFlow;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use luaur_ast::records::ast_stat_continue::AstStatContinue;

impl DataFlowGraphBuilder {
    pub fn visit_ast_stat_continue(&mut self, _c: *mut AstStatContinue) -> ControlFlow {
        ControlFlow::Continues
    }
}
