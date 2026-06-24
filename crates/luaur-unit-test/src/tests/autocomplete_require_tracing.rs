//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4490:autocomplete_require_tracing`
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
//!   - translates_to -> rust_item autocomplete_require_tracing

#[cfg(test)]
#[test]
fn autocomplete_require_tracing() {
    use crate::functions::null_callback_autocomplete_test::null_callback;
    use crate::records::ac_builtins_fixture::ACBuiltinsFixture;
    use luaur_ast::records::position::Position;

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.base.file_resolver.source.insert(
        String::from("Module/A"),
        String::from(
            r#"
return { x = 0 }
    "#,
        ),
    );
    fixture.base.base.file_resolver.source.insert(
        String::from("Module/B"),
        String::from(
            r#"
local result = require(script.Parent.A)
local x = 1 + result.
    "#,
        ),
    );

    let ac = fixture
        .base
        .autocomplete_module_name_position_string_completion_callback(
            &String::from("Module/B"),
            Position {
                line: 2,
                column: 21,
            },
            Box::new(null_callback),
        );

    assert_eq!(ac.entry_map.len(), 1);
    assert!(ac.entry_map.contains_key("x"));
}
