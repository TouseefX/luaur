use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_table::AstExprTable;

impl AstJsonEncoder {
    pub fn visit_ast_expr_table(&mut self, node: *mut AstExprTable) -> bool {
        self.write_ast_expr_table(node);
        false
    }
}
