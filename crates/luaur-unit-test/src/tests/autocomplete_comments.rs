//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2464:autocomplete_comments`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item autocomplete_comments

#[cfg(test)]
#[test]
fn autocomplete_comments() {
    use crate::functions::null_callback_autocomplete_test::null_callback;
    use crate::records::ac_fixture::AcFixture;
    use luaur_ast::records::position::Position;

    let mut fixture = AcFixture::default();
    fixture
        .base
        .base
        .file_resolver
        .source
        .insert(String::from("Comments"), String::from("--foo"));

    let module = String::from("Comments");
    let ac = fixture
        .base
        .autocomplete_module_name_position_string_completion_callback(
            &module,
            Position { line: 0, column: 5 },
            Box::new(null_callback),
        );

    assert_eq!(0, ac.entry_map.len());
}
