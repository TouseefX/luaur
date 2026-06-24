use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_type_table::AstTypeTable;

impl AstJsonEncoder {
    pub fn visit_ast_type_table(&mut self, node: *mut AstTypeTable) -> bool {
        self.write_ast_type_table(node);
        false
    }
}
