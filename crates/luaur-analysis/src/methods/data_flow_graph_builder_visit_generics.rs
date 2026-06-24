use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_generic_type::AstGenericType;

impl DataFlowGraphBuilder {
    pub fn visit_generics(&mut self, g: AstArray<*mut AstGenericType>) {
        for i in 0..g.size {
            let generic = unsafe { *g.data.add(i) };
            if generic.is_null() {
                continue;
            }

            let default_value = unsafe { (*generic).default_value };
            if !default_value.is_null() {
                self.visit_type_ast_type(default_value);
            }
        }
    }
}
