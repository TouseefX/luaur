use crate::records::json_encoder_fixture::JsonEncoderFixture;
use luaur_ast::records::parse_options::ParseOptions;
use luaur_ast::records::parse_result::ParseResult;
use luaur_ast::records::parser::Parser;

impl JsonEncoderFixture {
    pub fn parse(&mut self, src: &str) -> ParseResult {
        let mut options = ParseOptions::default();
        options.allow_declaration_syntax = true;

        self.names.rebind_allocator(&mut self.allocator as *mut _);

        Parser::parse(
            src,
            src.len(),
            &mut self.names,
            &mut self.allocator,
            options,
        )
    }
}
