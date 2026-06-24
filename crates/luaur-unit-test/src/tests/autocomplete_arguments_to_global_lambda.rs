//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1236:autocomplete_arguments_to_global_lambda`
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
//!   - translates_to -> rust_item autocomplete_arguments_to_global_lambda

#[cfg(test)]
#[test]
fn autocomplete_arguments_to_global_lambda() {
    use crate::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        abc = function(def, ghi@1)
        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac.entry_map.is_empty());
}
