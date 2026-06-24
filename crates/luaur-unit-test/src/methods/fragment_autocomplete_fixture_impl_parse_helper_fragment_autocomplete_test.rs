use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;
use alloc::string::String;
use luaur_analysis::records::source_module::SourceModule;
use luaur_ast::records::parse_options::ParseOptions;
use luaur_ast::records::parse_result::ParseResult;
use luaur_ast::records::parser::Parser;

impl FragmentAutocompleteFixtureImpl {
    pub fn parse_helper_(&mut self, source: &mut SourceModule, document: String) -> ParseResult {
        let mut parse_options = ParseOptions::default();
        parse_options.capture_comments = true;

        let allocator = alloc::sync::Arc::get_mut(&mut source.allocator)
            .expect("fresh fragment source allocator must be unique")
            as *mut _;
        let names = alloc::sync::Arc::get_mut(&mut source.names)
            .expect("fresh fragment source name table must be unique");
        names.rebind_allocator(allocator);

        let parse_result = unsafe {
            Parser::parse(
                document.as_str(),
                document.len(),
                names,
                &mut *allocator,
                parse_options,
            )
        };

        source.parse_errors = parse_result.errors.clone();
        source.root = parse_result.root;
        source.hotcomments = parse_result.hotcomments.clone();
        source.comment_locations = parse_result.comment_locations.clone();

        parse_result
    }
}
