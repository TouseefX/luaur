use crate::records::builtins_fixture::BuiltinsFixture;
use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct FragmentAutocompleteBuiltinsFixture {
    pub base: FragmentAutocompleteFixtureImpl,
}

impl Default for FragmentAutocompleteBuiltinsFixture {
    fn default() -> Self {
        Self::fragment_autocomplete_builtins_fixture()
    }
}
