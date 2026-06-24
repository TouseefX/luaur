//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:115:type_infer_definitions_definition_file_extern_types`
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
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - type_ref -> record Bar (tests/Variant.test.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
//!   - translates_to -> rust_item type_infer_definitions_definition_file_extern_types

#[cfg(test)]
#[test]
fn type_infer_definitions_definition_file_extern_types() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
        &String::from(
            r#"
        declare class Foo
            X: number

            function inheritance(self): number
        end

        declare class Bar extends Foo
            Y: number

            function foo(self, x: number): number
            function foo(self, x: string): string

            function __add(self, other: Bar): Bar
        end
    "#,
        ),
        false,
    );

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local x: Bar
        local prop: number = x.Y
        local inheritedProp: number = x.X
        local method: number = x:foo(1)
        local method2: string = x:foo("string")
        local metamethod: Bar = x + x
        local inheritedMethod: number = x:inheritance()
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("prop")))
    );
    assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("inheritedProp")))
    );
    assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("method")))
    );
    assert_eq!(
        "string",
        to_string_type_id(fixture.require_type_string(&String::from("method2")))
    );
    assert_eq!(
        "Bar",
        to_string_type_id(fixture.require_type_string(&String::from("metamethod")))
    );
    assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("inheritedMethod")))
    );
}
