//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2259:autocomplete_type_correct_suggestion_for_overloads`
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
//!   - translates_to -> rust_item autocomplete_type_correct_suggestion_for_overloads

#[cfg(test)]
#[test]
fn autocomplete_type_correct_suggestion_for_overloads() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::type_correct_kind::TypeCorrectKind;

    if !luaur_common::FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
local target: ((number) -> string) & ((string) -> number))

local one = 4
local two = "hello"
return target(o@1)
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac1.entry_map.contains_key("one"));
    assert_eq!(ac1.entry_map["one"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac1.entry_map["two"].type_correct, TypeCorrectKind::Correct);

    fixture.base.check(&String::from(
        r#"
local target: ((number) -> string) & ((number) -> number))

local one = 4
local two = "hello"
return target(o@1)
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac2.entry_map.contains_key("one"));
    assert_eq!(ac2.entry_map["one"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac2.entry_map["two"].type_correct, TypeCorrectKind::None);

    fixture.base.check(&String::from(
        r#"
local target: ((number, number) -> string) & ((string) -> number))

local one = 4
local two = "hello"
return target(1, o@1)
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac3.entry_map.contains_key("one"));
    assert_eq!(ac3.entry_map["one"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac3.entry_map["two"].type_correct, TypeCorrectKind::None);
}
