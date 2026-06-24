use crate::macros::prop::prop;
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_type_typeof::AstTypeTypeof;

impl AstJsonEncoder {
    pub fn visit_ast_type_typeof(&mut self, node: *mut AstTypeTypeof) -> bool {
        self.write_ast_type_typeof(node);
        false
    }
}
