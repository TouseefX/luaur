use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use luaur_ast::records::ast_type_pack_variadic::AstTypePackVariadic;

impl DataFlowGraphBuilder {
    pub fn visit_type_pack_ast_type_pack_variadic(&mut self, v: *mut AstTypePackVariadic) {
        unsafe {
            self.visit_type_ast_type((*v).variadic_type);
        }
    }
}
