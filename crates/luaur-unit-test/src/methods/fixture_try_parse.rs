//! Port of `Fixture::tryParse` (tests/Fixture.cpp:378). Unlike `parse`, this does
//! NOT throw/panic on parse errors — error-recovery tests use it to exercise the
//! parser and then inspect `result.errors` themselves. It enables declaration
//! syntax and parses into the fixture's own allocator + name table (which outlive
//! the returned AST for the test's duration).
use crate::records::fixture::Fixture;
use alloc::string::String;
use luaur_ast::records::parse_options::ParseOptions;
use luaur_ast::records::parse_result::ParseResult;
use luaur_ast::records::parser::Parser;

impl Fixture {
    pub fn try_parse(&mut self, source: &String, parse_options: &ParseOptions) -> ParseResult {
        let mut options: ParseOptions = parse_options.clone();
        options.allow_declaration_syntax = true;

        // See `fixture_parse.rs` — keep the name table pointed at the live allocator.
        self.name_table
            .rebind_allocator(&mut self.allocator as *mut _);

        Parser::parse(
            source.as_str(),
            source.len(),
            &mut self.name_table,
            &mut self.allocator,
            options,
        )
    }
}
