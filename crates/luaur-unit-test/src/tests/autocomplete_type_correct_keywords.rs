//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2217:autocomplete_type_correct_keywords`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> enum TypeCorrectKind (Analysis/include/Luau/AutocompleteTypes.h)
//!   - translates_to -> rust_item autocomplete_type_correct_keywords

#[cfg(test)]
#[test]
fn autocomplete_type_correct_keywords() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::type_correct_kind::TypeCorrectKind;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
local function a(x: boolean) end
local function b(x: number?) end
local function c(x: (number) -> string) end
local function d(x: ((number) -> string)?) end
local function e(x: ((number) -> string) & ((boolean) -> number)) end

local tru = {}
local ni = false

local ac = a(t@1)
local bc = b(n@2)
local cc = c(f@3)
local dc = d(f@4)
local ec = e(f@5)
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac1.entry_map.contains_key("tru"));
    assert_eq!(ac1.entry_map["tru"].type_correct, TypeCorrectKind::None);
    assert_eq!(ac1.entry_map["true"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(
        ac1.entry_map["false"].type_correct,
        TypeCorrectKind::Correct
    );

    let ac2 = fixture.base.autocomplete_marker(b'2' as core::ffi::c_char);
    assert!(ac2.entry_map.contains_key("ni"));
    assert_eq!(ac2.entry_map["ni"].type_correct, TypeCorrectKind::None);
    assert_eq!(ac2.entry_map["nil"].type_correct, TypeCorrectKind::Correct);

    let ac3 = fixture.base.autocomplete_marker(b'3' as core::ffi::c_char);
    assert!(ac3.entry_map.contains_key("false"));
    assert_eq!(ac3.entry_map["false"].type_correct, TypeCorrectKind::None);
    assert_eq!(
        ac3.entry_map["function"].type_correct,
        TypeCorrectKind::Correct
    );

    let ac4 = fixture.base.autocomplete_marker(b'4' as core::ffi::c_char);
    assert_eq!(
        ac4.entry_map["function"].type_correct,
        TypeCorrectKind::Correct
    );

    let ac5 = fixture.base.autocomplete_marker(b'5' as core::ffi::c_char);
    assert_eq!(
        ac5.entry_map["function"].type_correct,
        TypeCorrectKind::Correct
    );
}
