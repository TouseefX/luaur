//! C++ `FragmentTypeCheckResult FragmentAutocompleteFixtureImpl::checkFragment(...)`
//! (tests/FragmentAutocomplete.test.cpp:161-170).
use crate::functions::get_options::get_options;
use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;
use alloc::string::String;
use luaur_analysis::functions::typecheck_fragment_fragment_autocomplete_alt_b::typecheck_fragment;
use luaur_analysis::records::fragment_type_check_result::FragmentTypeCheckResult;
use luaur_analysis::records::i_fragment_autocomplete_reporter::null_reporter;
use luaur_ast::records::position::Position;

impl FragmentAutocompleteFixtureImpl {
    pub fn check_fragment(
        &mut self,
        document: &String,
        cursor_pos: Position,
        fragment_end_position: Option<Position>,
    ) -> FragmentTypeCheckResult {
        let p = self.parse_helper(document.clone());
        let options = get_options();
        let frontend = self.base.get_frontend();
        let (_, result) = typecheck_fragment(
            frontend,
            &String::from("MainModule"),
            &cursor_pos,
            Some(options),
            document.as_str(),
            fragment_end_position,
            p.root,
            // C++ passes `nullptr` for the reporter.
            null_reporter(),
        );
        result
    }
}
