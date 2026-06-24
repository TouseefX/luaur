use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_intersection::AstTypeIntersection;

impl DataFlowGraphBuilder {
    pub fn visit_type_ast_type_intersection(&mut self, i: *mut AstTypeIntersection) {
        unsafe {
            let types = (*i).types;
            for idx in 0..types.size {
                let t = *types.data.add(idx);
                self.visit_type_ast_type(t as *mut AstType);
            }
        }
    }
}
