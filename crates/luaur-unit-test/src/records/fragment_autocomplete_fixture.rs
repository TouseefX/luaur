use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct FragmentAutocompleteFixture {
    pub base: FragmentAutocompleteFixtureImpl,
}

impl Default for FragmentAutocompleteFixture {
    fn default() -> Self {
        Self::fragment_autocomplete_fixture()
    }
}
