//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2983:autocomplete_autocomplete_first_function_arg_expected_type`
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
//!   - translates_to -> rust_item autocomplete_autocomplete_first_function_arg_expected_type

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_first_function_arg_expected_type() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::type_correct_kind::TypeCorrectKind;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
local function foo1() return 1 end
local function foo2() return "1" end

local function bar0() return "got" .. a end
local function bar1(a: number) return "got " .. a end
local function bar2(a: number, b: string) return "got " .. a .. b end

local t = {}
function t:bar1(a: number) return "got " .. a end

local r1 = bar0(@1)
local r2 = bar1(@2)
local r3 = bar2(@3)
local r4 = t:bar1(@4)
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac1.entry_map.contains_key("foo1"));
    assert_eq!(ac1.entry_map["foo1"].type_correct, TypeCorrectKind::None);
    assert!(ac1.entry_map.contains_key("foo2"));
    assert_eq!(ac1.entry_map["foo2"].type_correct, TypeCorrectKind::None);

    let ac2 = fixture.base.autocomplete_marker(b'2' as core::ffi::c_char);

    assert!(ac2.entry_map.contains_key("foo1"));
    assert_eq!(
        ac2.entry_map["foo1"].type_correct,
        TypeCorrectKind::CorrectFunctionResult
    );
    assert!(ac2.entry_map.contains_key("foo2"));
    assert_eq!(ac2.entry_map["foo2"].type_correct, TypeCorrectKind::None);

    let ac3 = fixture.base.autocomplete_marker(b'3' as core::ffi::c_char);

    assert!(ac3.entry_map.contains_key("foo1"));
    assert_eq!(
        ac3.entry_map["foo1"].type_correct,
        TypeCorrectKind::CorrectFunctionResult
    );
    assert!(ac3.entry_map.contains_key("foo2"));
    assert_eq!(ac3.entry_map["foo2"].type_correct, TypeCorrectKind::None);

    let ac4 = fixture.base.autocomplete_marker(b'4' as core::ffi::c_char);

    assert!(ac4.entry_map.contains_key("foo1"));
    assert_eq!(
        ac4.entry_map["foo1"].type_correct,
        TypeCorrectKind::CorrectFunctionResult
    );
    assert!(ac4.entry_map.contains_key("foo2"));
    assert_eq!(ac4.entry_map["foo2"].type_correct, TypeCorrectKind::None);
}
