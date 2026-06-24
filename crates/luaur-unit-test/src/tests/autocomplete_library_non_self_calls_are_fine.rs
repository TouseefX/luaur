//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3653:autocomplete_library_non_self_calls_are_fine`
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
//!   - calls -> function getn (VM/src/ltablib.cpp)
//!   - translates_to -> rust_item autocomplete_library_non_self_calls_are_fine

#[cfg(test)]
#[test]
fn autocomplete_library_non_self_calls_are_fine() {
    use crate::records::ac_builtins_fixture::ACBuiltinsFixture;

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(&String::from(
        r#"
string.@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac.entry_map.contains_key("byte"));
    assert!(!ac.entry_map["byte"].wrong_index_type);
    assert!(!ac.entry_map["byte"].indexed_with_self);
    assert!(ac.entry_map.contains_key("char"));
    assert!(!ac.entry_map["char"].wrong_index_type);
    assert!(!ac.entry_map["char"].indexed_with_self);
    assert!(ac.entry_map.contains_key("sub"));
    assert!(!ac.entry_map["sub"].wrong_index_type);
    assert!(!ac.entry_map["sub"].indexed_with_self);

    fixture.base.check(&String::from(
        r#"
table.@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac.entry_map.contains_key("remove"));
    assert!(!ac.entry_map["remove"].wrong_index_type);
    assert!(!ac.entry_map["remove"].indexed_with_self);
    assert!(ac.entry_map.contains_key("getn"));
    assert!(!ac.entry_map["getn"].wrong_index_type);
    assert!(!ac.entry_map["getn"].indexed_with_self);
    assert!(ac.entry_map.contains_key("insert"));
    assert!(!ac.entry_map["insert"].wrong_index_type);
    assert!(!ac.entry_map["insert"].indexed_with_self);
}
