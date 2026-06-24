//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3505:autocomplete_simple`
//! Source: `tests/Autocomplete.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Autocomplete.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
//!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Common/include/Luau/StringUtils.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Autocomplete.test.cpp
//! - outgoing:
//!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
//!   - translates_to -> rust_item autocomplete_simple

#[cfg(test)]
#[test]
fn autocomplete_simple() {
    use crate::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
local t = {}
function t:m() end
t:m()
    "#,
    ));
}
