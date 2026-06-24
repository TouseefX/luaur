use crate::macros::prop::prop;
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_type_function::AstTypeFunction;

impl AstJsonEncoder {
    pub fn visit_ast_type_function(&mut self, node: *mut AstTypeFunction) -> bool {
        self.write_ast_type_function(node);
        false
    }
}
