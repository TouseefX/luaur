use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::location::Location;

impl AstJsonEncoder {
    pub fn write_location(&mut self, location: &Location) {
        self.write_raw_string_view("\"");
        self.write_position(&location.begin);
        self.write_raw_string_view(" - ");
        self.write_position(&location.end);
        self.write_raw_string_view("\"");
    }
}
