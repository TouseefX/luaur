use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber;

impl AstJsonEncoder {
    pub fn visit_ast_expr_constant_number(&mut self, node: *mut AstExprConstantNumber) -> bool {
        self.write_ast_expr_constant_number(node);
        false
    }
}
