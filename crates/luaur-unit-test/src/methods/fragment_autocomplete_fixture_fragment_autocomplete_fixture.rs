use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;

impl FragmentAutocompleteFixture {
    pub fn fragment_autocomplete_fixture() -> Self {
        Self {
            base: FragmentAutocompleteFixtureImpl::fragment_autocomplete_fixture_impl(),
        }
    }
}
