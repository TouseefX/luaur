use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;
use alloc::string::String;
use luaur_analysis::records::source_module::SourceModule;
use luaur_ast::records::parse_result::ParseResult;

impl FragmentAutocompleteFixtureImpl {
    pub fn parse_helper(&mut self, document: String) -> ParseResult {
        let source: *mut SourceModule = self.get_source();
        unsafe { self.parse_helper_(&mut *source, document) }
    }
}
