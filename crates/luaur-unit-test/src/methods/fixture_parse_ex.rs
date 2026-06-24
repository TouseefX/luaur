use crate::records::fixture::Fixture;
use alloc::string::String;
use alloc::vec::Vec;
use luaur_ast::records::parse_error::ParseError;
use luaur_ast::records::parse_options::ParseOptions;
use luaur_ast::records::parse_result::ParseResult;

impl Fixture {
    pub fn parse_ex(&mut self, source: &String, options: &ParseOptions) -> ParseResult {
        let result = self.try_parse(source, options);
        if !result.errors.is_empty() {
            panic!("ParseErrors: {:?}", result.errors);
        }
        result
    }
}
