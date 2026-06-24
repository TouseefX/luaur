use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_type_table::AstTypeTable;

impl NonStrictTypeChecker {
    pub fn visit_ast_type_table(&mut self, table: *mut AstTypeTable) {
        unsafe {
            if !(*table).indexer.is_null() {
                let indexer = (*table).indexer;
                self.visit_ast_type((*indexer).index_type);
                self.visit_ast_type((*indexer).result_type);
            }

            let props = &(*table).props;
            for i in 0..props.size {
                let prop = props.data.add(i);
                self.visit_ast_type((*prop).r#type);
            }
        }
    }
}
