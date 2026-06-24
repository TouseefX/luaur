//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1675:autocomplete_type_correct_local_type_suggestion`
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
//!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
//!   - translates_to -> rust_item autocomplete_type_correct_local_type_suggestion

#[cfg(test)]
#[test]
fn autocomplete_type_correct_local_type_suggestion() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::type_correct_kind::TypeCorrectKind;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
local b: s@1 = "str"
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac1.entry_map.contains_key("string"));
    assert_eq!(
        ac1.entry_map["string"].type_correct,
        TypeCorrectKind::Correct
    );

    fixture.base.check(&String::from(
        r#"
local function f() return "str" end
local b: s@1 = f()
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac2.entry_map.contains_key("string"));
    assert_eq!(
        ac2.entry_map["string"].type_correct,
        TypeCorrectKind::Correct
    );

    fixture.base.check(&String::from(
        r#"
local b: s@1, c: n@2 = "str", 2
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac3.entry_map.contains_key("string"));
    assert_eq!(
        ac3.entry_map["string"].type_correct,
        TypeCorrectKind::Correct
    );

    let ac4 = fixture.base.autocomplete_marker(b'2' as core::ffi::c_char);

    assert!(ac4.entry_map.contains_key("number"));
    assert_eq!(
        ac4.entry_map["number"].type_correct,
        TypeCorrectKind::Correct
    );

    fixture.base.check(&String::from(
        r#"
local function f() return 1, "str", 3 end
local a: b@1, b: n@2, c: s@3, d: n@4 = false, f()
    "#,
    ));

    let ac5 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac5.entry_map.contains_key("boolean"));
    assert_eq!(
        ac5.entry_map["boolean"].type_correct,
        TypeCorrectKind::Correct
    );

    let ac6 = fixture.base.autocomplete_marker(b'2' as core::ffi::c_char);

    assert!(ac6.entry_map.contains_key("number"));
    assert_eq!(
        ac6.entry_map["number"].type_correct,
        TypeCorrectKind::Correct
    );

    let ac7 = fixture.base.autocomplete_marker(b'3' as core::ffi::c_char);

    assert!(ac7.entry_map.contains_key("string"));
    assert_eq!(
        ac7.entry_map["string"].type_correct,
        TypeCorrectKind::Correct
    );

    let ac8 = fixture.base.autocomplete_marker(b'4' as core::ffi::c_char);

    assert!(ac8.entry_map.contains_key("number"));
    assert_eq!(
        ac8.entry_map["number"].type_correct,
        TypeCorrectKind::Correct
    );

    fixture.base.check(&String::from(
        r#"
local function f(): ...number return 1, 2, 3 end
local a: boolean, b: n@1 = false, f()
    "#,
    ));

    let ac9 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac9.entry_map.contains_key("number"));
    assert_eq!(
        ac9.entry_map["number"].type_correct,
        TypeCorrectKind::Correct
    );
}
