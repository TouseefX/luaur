use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;

impl AstJsonEncoder {
    pub fn visit_ast_expr_index_expr(&mut self, node: *mut AstExprIndexExpr) -> bool {
        self.write_ast_expr_index_expr(node);
        false
    }
}
