//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2068:autocomplete_do_not_suggest_internal_module_type`
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
//!   - translates_to -> rust_item autocomplete_do_not_suggest_internal_module_type

#[cfg(test)]
#[test]
fn autocomplete_do_not_suggest_internal_module_type() {
    use crate::functions::null_callback_autocomplete_test::null_callback;
    use crate::records::ac_fixture::AcFixture;
    use luaur_ast::records::position::Position;

    let mut fixture = AcFixture::default();
    fixture.base.base.file_resolver.source.insert(
        String::from("Module/A"),
        String::from(
            r#"
type done = { x: number, y: number }
local function a(a: (done) -> number) return a({x=1, y=2}) end
local function b(a: ((done) -> number) -> number) return a(function(done) return 1 end) end
return {a = a, b = b}
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
local ex = require(script.Parent.A)
ex.a(function(x:
    "#,
        ),
    );

    let module_b = String::from("Module/B");
    fixture
        .base
        .get_frontend()
        .check_module_name_optional_frontend_options(&module_b, None);

    let ac1 = fixture
        .base
        .autocomplete_module_name_position_string_completion_callback(
            &module_b,
            Position {
                line: 2,
                column: 16,
            },
            Box::new(null_callback),
        );

    assert!(!ac1.entry_map.contains_key("done"));

    fixture.base.base.file_resolver.source.insert(
        String::from("Module/C"),
        String::from(
            r#"
local ex = require(script.Parent.A)
ex.b(function(x:
    "#,
        ),
    );

    let module_c = String::from("Module/C");
    fixture
        .base
        .get_frontend()
        .check_module_name_optional_frontend_options(&module_c, None);

    let ac2 = fixture
        .base
        .autocomplete_module_name_position_string_completion_callback(
            &module_c,
            Position {
                line: 2,
                column: 16,
            },
            Box::new(null_callback),
        );

    assert!(!ac2.entry_map.contains_key("(done) -> number"));
}
