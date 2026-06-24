use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use luaur_ast::records::ast_type_function::AstTypeFunction;

impl DataFlowGraphBuilder {
    pub fn visit_type_ast_type_function(&mut self, f: *mut AstTypeFunction) {
        unsafe {
            let f_ref = &*f;

            self.visit_generics(f_ref.generics);
            self.visit_generic_packs(f_ref.generic_packs);
            self.visit_type_list(f_ref.arg_types);
            self.visit_type_pack_ast_type_pack(f_ref.return_types);
        }
    }
}
