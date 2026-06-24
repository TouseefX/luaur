//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4613:autocomplete_autocomplete_in_local_table`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Entry (Ast/include/Luau/Lexer.h)
//!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item autocomplete_autocomplete_in_local_table

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_in_local_table() {
    use crate::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        type Entry = { field: number, prop: string }
        local x : {Entry} = {}
        x[1] = {
           f@1,
           p@2,
        }

        local t : { key1: boolean, thing2: CFrame, aaa3: vector } = {
            k@3,
            th@4,
        }
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(ac1.entry_map.get("field").is_some(), true);
    let ac2 = fixture.base.autocomplete_marker(b'2' as core::ffi::c_char);
    assert_eq!(ac2.entry_map.get("prop").is_some(), true);
    let ac3 = fixture.base.autocomplete_marker(b'3' as core::ffi::c_char);
    assert_eq!(ac3.entry_map.get("key1").is_some(), true);
    let ac4 = fixture.base.autocomplete_marker(b'4' as core::ffi::c_char);
    assert_eq!(ac4.entry_map.get("thing2").is_some(), true);
}
