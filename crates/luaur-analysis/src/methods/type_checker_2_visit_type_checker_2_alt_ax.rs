use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_type_table::AstTypeTable;

impl TypeChecker2 {
    pub fn visit_ast_type_table(&mut self, table: *mut AstTypeTable) {
        unsafe {
            let table = &*table;

            for i in 0..table.props.size {
                let prop = &*table.props.data.add(i);
                self.visit_ast_type(prop.r#type);
            }

            if !table.indexer.is_null() {
                let indexer = &*table.indexer;
                self.visit_ast_type(indexer.index_type);
                self.visit_ast_type(indexer.result_type);
            }
        }
    }
}
