use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use luaur_ast::records::ast_type_union::AstTypeUnion;

impl DataFlowGraphBuilder {
    pub fn visit_type_ast_type_union(&mut self, u: *mut AstTypeUnion) {
        unsafe {
            let types = (*u).types;
            for i in 0..types.size {
                let t = *types.data.add(i);
                self.visit_type_ast_type(t);
            }
        }
    }
}
