//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1247:autocomplete_function_expr_params`
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
//!   - translates_to -> rust_item autocomplete_function_expr_params

#[cfg(test)]
#[test]
fn autocomplete_function_expr_params() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        abc = function(def) @1
    "#,
    ));

    for i in 20..27 {
        assert!(fixture
            .base
            .autocomplete_position(1, i)
            .entry_map
            .is_empty());
    }
    assert!(!fixture
        .base
        .autocomplete_marker(b'1' as core::ffi::c_char)
        .entry_map
        .is_empty());

    fixture.base.check(&String::from(
        r#"
        abc = function(def) @1
        end
    "#,
    ));

    for i in 20..27 {
        assert!(fixture
            .base
            .autocomplete_position(1, i)
            .entry_map
            .is_empty());
    }
    assert!(!fixture
        .base
        .autocomplete_marker(b'1' as core::ffi::c_char)
        .entry_map
        .is_empty());

    fixture.base.check(&String::from(
        r#"
        abc = function(def)
@1
        end
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(1, ac2.entry_map.get("def").map_or(0, |_| 1));
    assert_eq!(ac2.context, AutocompleteContext::Statement);
}
