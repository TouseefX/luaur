use crate::records::fixture::Fixture;

impl Fixture {
    /// C++: `explicit Fixture(bool prepareAutocomplete = false) : forAutocomplete(prepareAutocomplete) {}`
    /// (tests/Fixture.cpp:263). An associated constructor — every other member is
    /// built by `Default`, then `for_autocomplete` is set to the requested value.
    pub fn fixture_bool(prepare_autocomplete: bool) -> Self {
        let mut fixture = Self::default();
        fixture.for_autocomplete = prepare_autocomplete;
        fixture
    }
}
