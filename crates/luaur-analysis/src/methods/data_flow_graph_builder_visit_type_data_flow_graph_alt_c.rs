use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use luaur_ast::records::ast_table_prop::AstTableProp;
use luaur_ast::records::ast_type_table::AstTypeTable;

impl DataFlowGraphBuilder {
    pub fn visit_type_ast_type_table(&mut self, t: *mut AstTypeTable) {
        unsafe {
            let table = &*t;
            for i in 0..table.props.size {
                let prop = &*table.props.data.add(i);
                self.visit_type_ast_type(prop.r#type);
            }

            if !table.indexer.is_null() {
                let indexer = &*table.indexer;
                self.visit_type_ast_type(indexer.index_type);
                self.visit_type_ast_type(indexer.result_type);
            }
        }
    }
}
