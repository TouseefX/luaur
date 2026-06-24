//! C++ `FragmentAutocompleteStatusResult
//! FragmentAutocompleteFixtureImpl::autocompleteFragmentForModule(...)`
//! (tests/FragmentAutocomplete.test.cpp:270-283).
use crate::functions::null_callback_autocomplete_test::null_callback;
use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;
use alloc::boxed::Box;
use alloc::string::String;
use luaur_analysis::functions::try_fragment_autocomplete::try_fragment_autocomplete;
use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
use luaur_analysis::records::fragment_context::FragmentContext;
use luaur_analysis::records::frontend_options::FrontendOptions;
use luaur_analysis::type_aliases::module_name_file_resolver::ModuleName;
use luaur_ast::records::position::Position;

impl FragmentAutocompleteFixtureImpl {
    pub fn autocomplete_fragment_for_module(
        &mut self,
        module: &ModuleName,
        document: &String,
        cursor_pos: Position,
        fragment_end_position: Option<Position>,
    ) -> FragmentAutocompleteStatusResult {
        let parse_result = self.parse_helper(document.clone());
        // C++ uses a default-constructed FrontendOptions here (not getOptions()).
        let options = FrontendOptions::default();
        let context = FragmentContext::new_with_options(
            document.as_str(),
            &parse_result,
            Some(options),
            fragment_end_position,
        );
        let frontend = self.base.get_frontend();
        try_fragment_autocomplete(
            frontend,
            module,
            cursor_pos,
            context,
            Box::new(null_callback),
        )
    }
}
