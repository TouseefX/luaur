use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use luaur_ast::records::ast_type_typeof::AstTypeTypeof;

impl DataFlowGraphBuilder {
    pub fn visit_type_ast_type_typeof(&mut self, t: *mut AstTypeTypeof) {
        unsafe {
            self.visit_expr_ast_expr((*t).expr);
        }
    }
}
