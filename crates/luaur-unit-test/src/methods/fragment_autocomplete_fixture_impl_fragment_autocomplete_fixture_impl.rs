use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;

impl FragmentAutocompleteFixtureImpl {
    pub fn fragment_autocomplete_fixture_impl() -> Self {
        let mut base = crate::records::builtins_fixture::BuiltinsFixture::default();
        base.builtins_fixture_builtins_fixture(true);
        FragmentAutocompleteFixtureImpl {
            base,
            marker_position: std::collections::BTreeMap::new(),
        }
    }
}
