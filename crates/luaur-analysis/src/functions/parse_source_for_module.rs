use crate::records::source_module::SourceModule;
use luaur_ast::enums::mode::Mode;
use luaur_ast::records::parse_options::ParseOptions;
use luaur_ast::records::parse_result::ParseResult;
use luaur_ast::records::parser::Parser;

pub fn parse_source_for_module(
    source: &str,
    source_module: &mut SourceModule,
    capture_comments: bool,
) -> ParseResult {
    let mut options = ParseOptions::default();
    options.allow_declaration_syntax = true;
    options.capture_comments = capture_comments;

    let parse_result = Parser::parse(
        source,
        source.len(),
        alloc::sync::Arc::get_mut(&mut source_module.names)
            .expect("SourceModule names must be uniquely owned while parsing"),
        alloc::sync::Arc::get_mut(&mut source_module.allocator)
            .expect("SourceModule allocator must be uniquely owned while parsing"),
        options.clone(),
    );

    source_module.root = parse_result.root;
    source_module.mode = Some(Mode::Definition);

    if options.capture_comments {
        source_module.hotcomments = parse_result.hotcomments.clone();
        source_module.comment_locations = parse_result.comment_locations.clone();
    }

    parse_result
}
