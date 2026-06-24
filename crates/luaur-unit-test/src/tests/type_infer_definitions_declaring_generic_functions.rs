//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:240:type_infer_definitions_declaring_generic_functions`
//! Source: `tests/TypeInfer.definitions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_definitions_declaring_generic_functions

#[cfg(test)]
#[test]
fn type_infer_definitions_declaring_generic_functions() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
        &String::from(
            r#"
        declare function f<a, b>(a: a, b: b): string
        declare function g<a..., b...>(...: a...): b...
        declare function h<a, b>(a: a, b: b): (b, a)
    "#,
        ),
        false,
    );

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local x = f(1, true)
        local y: number, z: string = g("foo", 123)
        local w, u = h(1, true)

        local f = f
        local g = g
        local h = h
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "string",
        to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
    assert_eq!(
        "boolean",
        to_string_type_id(fixture.require_type_string(&String::from("w")))
    );
    assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("u")))
    );
    assert_eq!(
        "<a, b>(a, b) -> string",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
    assert_eq!(
        "<a..., b...>(a...) -> (b...)",
        to_string_type_id(fixture.require_type_string(&String::from("g")))
    );
    assert_eq!(
        "<a, b>(a, b) -> (b, a)",
        to_string_type_id(fixture.require_type_string(&String::from("h")))
    );
}
