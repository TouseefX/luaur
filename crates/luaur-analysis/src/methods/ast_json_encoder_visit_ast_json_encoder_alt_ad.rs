use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_expr::AstStatExpr;

impl AstJsonEncoder {
    pub fn visit_ast_stat_expr(&mut self, node: *mut AstStatExpr) -> bool {
        unsafe {
            self.write_ast_stat_expr(node);
        }
        false
    }
}
