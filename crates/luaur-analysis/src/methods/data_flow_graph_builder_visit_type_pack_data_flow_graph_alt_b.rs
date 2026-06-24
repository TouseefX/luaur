use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit;

impl DataFlowGraphBuilder {
    pub fn visit_type_pack_ast_type_pack_explicit(&mut self, e: *mut AstTypePackExplicit) {
        unsafe {
            let type_list = (*e).type_list;
            self.visit_type_list(type_list);
        }
    }
}
