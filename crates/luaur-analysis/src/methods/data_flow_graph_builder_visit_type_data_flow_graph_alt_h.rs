use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use luaur_ast::records::ast_type_error::AstTypeError;

impl DataFlowGraphBuilder {
    pub fn visit_type_ast_type_error(&mut self, error: *mut AstTypeError) {
        unsafe {
            let types = (*error).types;
            for i in 0..types.size {
                let t = *types.data.add(i);
                self.visit_type_ast_type(t);
            }
        }
    }
}
