use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_function::AstExprFunction;

impl AstJsonEncoder {
    pub fn visit_ast_expr_function(&mut self, node: *mut AstExprFunction) -> bool {
        self.write_ast_expr_function(node);
        false
    }
}
