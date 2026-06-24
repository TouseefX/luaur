//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1192:autocomplete_global_function_params`
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
//!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
//!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
//!   - translates_to -> rust_item autocomplete_global_function_params

#[cfg(test)]
#[test]
fn autocomplete_global_function_params() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        function abc(def)
    "#,
    ));

    for i in 17..25 {
        assert!(fixture
            .base
            .autocomplete_position(1, i)
            .entry_map
            .is_empty());
    }
    assert!(!fixture
        .base
        .autocomplete_position(1, 26)
        .entry_map
        .is_empty());

    fixture.base.check(&String::from(
        r#"
        function abc(def)
        end
    "#,
    ));

    for i in 17..25 {
        assert!(fixture
            .base
            .autocomplete_position(1, i)
            .entry_map
            .is_empty());
    }
    assert!(!fixture
        .base
        .autocomplete_position(1, 26)
        .entry_map
        .is_empty());

    fixture.base.check(&String::from(
        r#"
        function abc(def)
@1
        end
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(1, ac2.entry_map.get("abc").map_or(0, |_| 1));
    assert_eq!(1, ac2.entry_map.get("def").map_or(0, |_| 1));
    assert_eq!(ac2.context, AutocompleteContext::Statement);

    fixture.base.check(&String::from(
        r#"
        function abc(def, ghi@1)
        end
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac3.entry_map.is_empty());
    assert_eq!(ac3.context, AutocompleteContext::Unknown);
}
