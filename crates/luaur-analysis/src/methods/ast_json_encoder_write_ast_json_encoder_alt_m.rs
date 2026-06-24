use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_name::AstName;

impl AstJsonEncoder {
    pub fn write_optional_ast_name(&mut self, name: Option<AstName>) {
        if let Some(name) = name {
            self.write_ast_name(name);
        } else {
            self.write_raw_string_view("null");
        }
    }
}
