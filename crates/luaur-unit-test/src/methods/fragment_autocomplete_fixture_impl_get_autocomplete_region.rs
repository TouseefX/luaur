use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;
use alloc::string::String;
use luaur_analysis::functions::get_fragment_region::get_fragment_region;
use luaur_analysis::records::fragment_region::FragmentRegion;
use luaur_ast::records::parse_result::ParseResult;
use luaur_ast::records::position::Position;

impl FragmentAutocompleteFixtureImpl {
    pub fn get_autocomplete_region(
        &mut self,
        source: String,
        cursor_pos: &Position,
    ) -> FragmentRegion {
        let parse_result: ParseResult = self.parse_helper(source);
        get_fragment_region(parse_result.root, cursor_pos)
    }
}
