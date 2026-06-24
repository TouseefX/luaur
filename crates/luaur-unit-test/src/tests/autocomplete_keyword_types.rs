//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2439:autocomplete_keyword_types`
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
//!   - calls -> method ACFixture::getFrontend (tests/Autocomplete.test.cpp)
//!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item autocomplete_keyword_types

#[cfg(test)]
#[test]
fn autocomplete_keyword_types() {
    use crate::functions::null_callback_autocomplete_test::null_callback;
    use crate::records::ac_fixture::AcFixture;
    use luaur_ast::records::position::Position;

    let mut fixture = AcFixture::default();
    fixture.base.base.file_resolver.source.insert(
        String::from("Module/A"),
        String::from(
            r#"
export type done = { x: number, y: number }
export type other = { z: number, w: number }
return {}
    "#,
        ),
    );

    let module_a = String::from("Module/A");
    let result = fixture
        .base
        .get_frontend()
        .check_module_name_optional_frontend_options(&module_a, None);
    assert!(result.errors.is_empty());

    fixture.base.base.file_resolver.source.insert(
        String::from("Module/B"),
        String::from(
            r#"
local aaa = require(script.Parent.A)
local a: aaa.do
    "#,
        ),
    );

    let module_b = String::from("Module/B");
    fixture
        .base
        .get_frontend()
        .check_module_name_optional_frontend_options(&module_b, None);

    let ac = fixture
        .base
        .autocomplete_module_name_position_string_completion_callback(
            &module_b,
            Position {
                line: 2,
                column: 15,
            },
            Box::new(null_callback),
        );

    assert_eq!(2, ac.entry_map.len());
    assert!(ac.entry_map.contains_key("done"));
    assert!(ac.entry_map.contains_key("other"));
}
