use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::type_aliases::ast_argument_name::AstArgumentName;

impl AstJsonEncoder {
    pub fn write_optional_ast_argument_name(&mut self, name: Option<AstArgumentName>) {
        if let Some(n) = name {
            self.write_ast_argument_name(n);
        } else {
            self.write_raw_string_view("null");
        }
    }
}
