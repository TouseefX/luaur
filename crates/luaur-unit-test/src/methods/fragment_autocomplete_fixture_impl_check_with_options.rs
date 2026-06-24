use crate::functions::get_options::get_options;
use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;
use alloc::string::String;
use luaur_analysis::records::check_result::CheckResult;

impl FragmentAutocompleteFixtureImpl {
    pub fn check_with_options(&mut self, source: &String) -> CheckResult {
        self.base.get_frontend();
        self.base
            .base
            .check_string_optional_frontend_options(source, Some(get_options()))
    }
}
