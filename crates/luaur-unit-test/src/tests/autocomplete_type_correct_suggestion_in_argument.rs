//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1528:autocomplete_type_correct_suggestion_in_argument`
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
//!   - calls -> function match (VM/src/lstrlib.cpp)
//!   - translates_to -> rust_item autocomplete_type_correct_suggestion_in_argument

#[cfg(test)]
#[test]
fn autocomplete_type_correct_suggestion_in_argument() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::type_correct_kind::TypeCorrectKind;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
local function target(a: number, b: string) return a + #b end

local one = 4
local two = "hello"
return target(o@1
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac1.entry_map.contains_key("one"));
    assert_eq!(ac1.entry_map["one"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac1.entry_map["two"].type_correct, TypeCorrectKind::None);

    fixture.base.check(&String::from(
        r#"
local function target(a: number, b: string) return a + #b end

local one = 4
local two = "hello"
return target(one, t@1
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac2.entry_map.contains_key("two"));
    assert_eq!(ac2.entry_map["two"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac2.entry_map["one"].type_correct, TypeCorrectKind::None);

    fixture.base.check(&String::from(
        r#"
local function target(a: number, b: string) return a + #b end

local a = { one = 4, two = "hello" }
return target(a.@1
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac3.entry_map.contains_key("one"));
    assert_eq!(ac3.entry_map["one"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac3.entry_map["two"].type_correct, TypeCorrectKind::None);

    fixture.base.check(&String::from(
        r#"
local function target(a: number, b: string) return a + #b end

local a = { one = 4, two = "hello" }
return target(a.one, a.@1
    "#,
    ));

    let ac4 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac4.entry_map.contains_key("two"));
    assert_eq!(ac4.entry_map["two"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac4.entry_map["one"].type_correct, TypeCorrectKind::None);

    fixture.base.check(&String::from(
        r#"
local function target(a: string?) return #b end

local a = { one = 4, two = "hello" }
return target(a.@1
    "#,
    ));

    let ac5 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac5.entry_map.contains_key("two"));
    assert_eq!(ac5.entry_map["two"].type_correct, TypeCorrectKind::Correct);
    assert_eq!(ac5.entry_map["one"].type_correct, TypeCorrectKind::None);
}
