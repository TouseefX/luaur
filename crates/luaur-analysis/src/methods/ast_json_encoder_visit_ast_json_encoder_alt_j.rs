use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_interp_string::AstExprInterpString;

impl AstJsonEncoder {
    pub fn visit_ast_expr_interp_string(&mut self, _node: *mut AstExprInterpString) -> bool {
        self.write_ast_expr_interp_string(_node);
        false
    }
}
