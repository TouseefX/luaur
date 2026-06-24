use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_type_error::AstTypeError;

impl AstJsonEncoder {
    pub fn visit_ast_type_error(&mut self, node: *mut AstTypeError) -> bool {
        self.write_ast_type_error(node);
        false
    }
}
