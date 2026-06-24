use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_generic_type_pack::AstGenericTypePack;

impl DataFlowGraphBuilder {
    pub fn visit_generic_packs(&mut self, g: AstArray<*mut AstGenericTypePack>) {
        for i in 0..g.size {
            let generic = unsafe { *g.data.add(i) };
            if !generic.is_null() && unsafe { (*generic).default_value }.is_null() == false {
                self.visit_type_pack_ast_type_pack(unsafe { (*generic).default_value });
            }
        }
    }
}
