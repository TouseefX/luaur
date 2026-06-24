use crate::functions::get_options::get_options;
use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;
use alloc::string::String;
use luaur_analysis::functions::parse_fragment::parse_fragment;
use luaur_analysis::records::fragment_parse_result::FragmentParseResult;
use luaur_ast::records::position::Position;

impl FragmentAutocompleteFixtureImpl {
    pub fn parse_fragment(
        &mut self,
        document: &String,
        cursor_pos: &Position,
        fragment_end_position: Option<Position>,
    ) -> Option<FragmentParseResult> {
        let parse_result = self.parse_helper(document.clone());
        let module = self
            .base
            .base
            .get_main_module(get_options().for_autocomplete);

        if module.is_null() {
            return None;
        }

        let module_ref = unsafe { &mut *module };
        let Some(names) = module_ref.names.as_ref() else {
            return None;
        };
        let names = alloc::sync::Arc::as_ptr(names) as *mut _;

        parse_fragment(
            module_ref.root,
            parse_result.root,
            names,
            document.as_str(),
            cursor_pos,
            fragment_end_position,
        )
    }
}
