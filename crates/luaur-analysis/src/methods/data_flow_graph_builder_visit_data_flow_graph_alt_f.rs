use crate::enums::control_flow::ControlFlow;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use luaur_ast::records::ast_stat_break::AstStatBreak;

impl DataFlowGraphBuilder {
    pub fn visit_ast_stat_break(&mut self, _b: *mut AstStatBreak) -> ControlFlow {
        ControlFlow::Breaks
    }
}
