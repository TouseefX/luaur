use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use luaur_ast::records::ast_type_or_pack::AstTypeOrPack;
use luaur_ast::records::ast_type_reference::AstTypeReference;

impl DataFlowGraphBuilder {
    pub fn visit_type_ast_type_reference(&mut self, r: *mut AstTypeReference) {
        let parameters = unsafe { (*r).parameters };
        for i in 0..parameters.size {
            let param: AstTypeOrPack = unsafe { *parameters.data.add(i) };
            if !param.r#type.is_null() {
                self.visit_type_ast_type(param.r#type);
            } else {
                self.visit_type_pack_ast_type_pack(param.type_pack);
            }
        }
    }
}
