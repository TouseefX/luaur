//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3977:autocomplete_string_completion_outside_quotes`
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
//!   - calls -> method ACFixtureImpl::loadDefinition (tests/Autocomplete.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record GlobalTypes (Analysis/include/Luau/GlobalTypes.h)
//!   - calls -> method ACFixture::getFrontend (tests/Autocomplete.test.cpp)
//!   - calls -> function linearSearchForBinding (tests/Fixture.cpp)
//!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
//!   - type_ref -> type_alias StringCompletionCallback (Analysis/include/Luau/AutocompleteTypes.h)
//!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
//!   - type_ref -> type_alias AutocompleteEntryMap (Analysis/include/Luau/AutocompleteTypes.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - type_ref -> record AutocompleteEntry (Analysis/include/Luau/AutocompleteTypes.h)
//!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
//!   - translates_to -> rust_item autocomplete_string_completion_outside_quotes

#[cfg(test)]
#[test]
fn autocomplete_string_completion_outside_quotes() {
    use crate::functions::autocomplete_attach_require_call_tag::autocomplete_attach_require_call_tag;
    use crate::records::ac_fixture::AcFixture;
    use alloc::string::String;
    use luaur_analysis::enums::autocomplete_entry_kind::AutocompleteEntryKind;
    use luaur_analysis::records::autocomplete_entry::AutocompleteEntry;
    use luaur_analysis::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;

    let mut fixture = AcFixture::default();
    fixture.base.load_definition(&String::from(
        r#"
        declare function require(path: string): any
    "#,
    ));

    autocomplete_attach_require_call_tag(fixture.base.get_frontend());

    fixture.base.check(&String::from(
        r#"
        local x = require(@1"@2"@3)
    "#,
    ));

    let ac = fixture.base.autocomplete_marker_callback(
        b'2' as core::ffi::c_char,
        Box::new(|_tag, _extern_type, _contents| {
            let mut results = AutocompleteEntryMap::new();
            results.insert(
                String::from("test"),
                AutocompleteEntry {
                    kind: AutocompleteEntryKind::String,
                    ..Default::default()
                },
            );
            Some(results)
        }),
    );

    assert_eq!(ac.entry_map.len(), 1);
    assert!(ac.entry_map.contains_key("test"));

    let ac = fixture.base.autocomplete_marker_callback(
        b'1' as core::ffi::c_char,
        Box::new(|_tag, _extern_type, _contents| {
            let mut results = AutocompleteEntryMap::new();
            results.insert(
                String::from("test"),
                AutocompleteEntry {
                    kind: AutocompleteEntryKind::String,
                    ..Default::default()
                },
            );
            Some(results)
        }),
    );

    assert_eq!(ac.entry_map.len(), 0);

    let ac = fixture.base.autocomplete_marker_callback(
        b'3' as core::ffi::c_char,
        Box::new(|_tag, _extern_type, _contents| {
            let mut results = AutocompleteEntryMap::new();
            results.insert(
                String::from("test"),
                AutocompleteEntry {
                    kind: AutocompleteEntryKind::String,
                    ..Default::default()
                },
            );
            Some(results)
        }),
    );

    assert_eq!(ac.entry_map.len(), 0);
}
