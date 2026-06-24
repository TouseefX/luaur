use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_varargs::AstExprVarargs;

impl AstJsonEncoder {
    pub fn visit_ast_expr_varargs(&mut self, node: *mut AstExprVarargs) -> bool {
        self.write_ast_expr_varargs(node);
        false
    }
}
