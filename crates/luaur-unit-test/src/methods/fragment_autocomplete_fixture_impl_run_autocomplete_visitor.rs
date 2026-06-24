use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;
use alloc::string::String;
use luaur_analysis::functions::find_ancestry_for_fragment_parse::find_ancestry_for_fragment_parse;
use luaur_analysis::records::fragment_autocomplete_ancestry_result::FragmentAutocompleteAncestryResult;
use luaur_ast::records::parse_options::ParseOptions;
use luaur_ast::records::position::Position;

impl FragmentAutocompleteFixtureImpl {
    pub fn run_autocomplete_visitor(
        &mut self,
        source: &String,
        cursor_pos: &Position,
    ) -> FragmentAutocompleteAncestryResult {
        let parse_result = self.base.base.try_parse(source, &ParseOptions::default());
        assert!(!parse_result.root.is_null());
        find_ancestry_for_fragment_parse(parse_result.root, *cursor_pos, parse_result.root)
    }
}
