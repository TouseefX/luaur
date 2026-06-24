use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_type_list::AstTypeList;

impl AstJsonEncoder {
    pub fn write_optional_ast_type_list(&mut self, type_list: Option<AstTypeList>) {
        if let Some(type_list) = type_list {
            self.write_ast_type_list(&type_list);
        } else {
            self.write_string_view("null");
        }
    }
}
