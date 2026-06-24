//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2472:autocomplete_autocomplete_prop_index_function_metamethod_is_variadic`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item autocomplete_autocomplete_prop_index_function_metamethod_is_variadic

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_prop_index_function_metamethod_is_variadic() {
    use crate::functions::null_callback_autocomplete_test::null_callback;
    use crate::records::ac_builtins_fixture::ACBuiltinsFixture;
    use luaur_ast::records::position::Position;

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.base.file_resolver.source.insert(
        String::from("Module/A"),
        String::from(
            r#"
        type Foo = {x: number}
        local t = {}
        setmetatable(t, {
            __index = function(index: string): ...Foo
                return {x = 1}, {x = 2}
            end
        })

        local a = t. -- Line 9
        --          | Column 20
    "#,
        ),
    );

    let module = String::from("Module/A");
    let ac = fixture
        .base
        .autocomplete_module_name_position_string_completion_callback(
            &module,
            Position {
                line: 9,
                column: 20,
            },
            Box::new(null_callback),
        );

    assert_eq!(1, ac.entry_map.len());
    assert!(ac.entry_map.contains_key("x"));
}
