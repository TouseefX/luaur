use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use luaur_ast::records::ast_type_list::AstTypeList;

impl DataFlowGraphBuilder {
    pub fn visit_type_list(&mut self, l: AstTypeList) {
        for i in 0..l.types.size {
            let t = unsafe { *l.types.data.add(i) };
            self.visit_type_ast_type(t);
        }

        if !l.tail_type.is_null() {
            self.visit_type_pack_ast_type_pack(l.tail_type);
        }
    }
}
