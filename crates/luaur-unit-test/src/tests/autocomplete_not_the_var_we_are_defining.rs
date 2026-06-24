//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2576:autocomplete_not_the_var_we_are_defining`
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
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item autocomplete_not_the_var_we_are_defining

#[cfg(test)]
#[test]
fn autocomplete_not_the_var_we_are_defining() {
    use crate::functions::null_callback_autocomplete_test::null_callback;
    use crate::records::ac_fixture::AcFixture;
    use luaur_ast::records::position::Position;

    let mut fixture = AcFixture::default();
    fixture
        .base
        .base
        .file_resolver
        .source
        .insert(String::from("Module/A"), String::from("abc,de"));

    let module = String::from("Module/A");
    let ac = fixture
        .base
        .autocomplete_module_name_position_string_completion_callback(
            &module,
            Position { line: 0, column: 6 },
            Box::new(null_callback),
        );

    assert!(!ac.entry_map.contains_key("de"));
}
