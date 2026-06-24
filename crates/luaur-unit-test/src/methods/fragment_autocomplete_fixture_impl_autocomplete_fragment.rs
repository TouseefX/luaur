//! C++ `FragmentAutocompleteStatusResult FragmentAutocompleteFixtureImpl::autocompleteFragment(...)`
//! (tests/FragmentAutocomplete.test.cpp:172-184).
use crate::functions::get_options::get_options;
use crate::functions::null_callback_autocomplete_test::null_callback;
use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;
use alloc::boxed::Box;
use alloc::string::String;
use luaur_analysis::functions::try_fragment_autocomplete::try_fragment_autocomplete;
use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
use luaur_analysis::records::fragment_context::FragmentContext;
use luaur_ast::records::position::Position;

impl FragmentAutocompleteFixtureImpl {
    pub fn autocomplete_fragment(
        &mut self,
        document: &String,
        cursor_pos: Position,
        fragment_end_position: Option<Position>,
    ) -> FragmentAutocompleteStatusResult {
        let parse_result = self.parse_helper(document.clone());
        let options = get_options();
        let context = FragmentContext::new_with_options(
            document.as_str(),
            &parse_result,
            Some(options),
            fragment_end_position,
        );
        let frontend = self.base.get_frontend();
        try_fragment_autocomplete(
            frontend,
            &String::from("MainModule"),
            cursor_pos,
            context,
            Box::new(null_callback),
        )
    }
}
