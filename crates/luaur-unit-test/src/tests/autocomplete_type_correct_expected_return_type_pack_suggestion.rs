//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1998:autocomplete_type_correct_expected_return_type_pack_suggestion`
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
//!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
//!   - translates_to -> rust_item autocomplete_type_correct_expected_return_type_pack_suggestion

#[cfg(test)]
#[test]
fn autocomplete_type_correct_expected_return_type_pack_suggestion() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::type_correct_kind::TypeCorrectKind;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
local function target(callback: () -> ...number) return callback() end

local x = target(function(): ...n@1
    return 1, 2, 3
end
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac1.entry_map.contains_key("number"));
    assert_eq!(
        ac1.entry_map["number"].type_correct,
        TypeCorrectKind::Correct
    );

    fixture.base.check(&String::from(
        r#"
local function target(callback: () -> ...number) return callback() end

local x = target(function(): (number, number, ...n@1
    return 1, 2, 3
end
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac2.entry_map.contains_key("number"));
    assert_eq!(
        ac2.entry_map["number"].type_correct,
        TypeCorrectKind::Correct
    );
}
