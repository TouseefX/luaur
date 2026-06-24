//! C++ `const Position& FragmentAutocompleteFixtureImpl::getPosition(char marker) const`
//! (tests/FragmentAutocomplete.test.cpp:291-296).
use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;
use luaur_ast::records::position::Position;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl FragmentAutocompleteFixtureImpl {
    pub fn get_position(&self, marker: char) -> Position {
        let found = self.marker_position.get(&marker);
        LUAU_ASSERT!(found.is_some());
        *found.unwrap()
    }
}
