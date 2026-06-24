//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:288:type_infer_definitions_definition_file_class_function_args`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
//!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
//!   - translates_to -> rust_item type_infer_definitions_definition_file_class_function_args

#[cfg(test)]
#[test]
fn type_infer_definitions_definition_file_class_function_args() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
        &String::from(
            r#"
        declare class Foo
            function foo1(self, x: number): number
            function foo2(self, x: number, y: string): number

            y: (a: number, b: string) -> string
        end

        declare Foo: {
            new: () -> Foo
        }
    "#,
        ),
        false,
    );

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local x: Foo = Foo.new()
        local methodRef1 = x.foo1
        local methodRef2 = x.foo2
        local prop = x.y
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions::default();
    opts.function_type_arguments = true;
    assert_eq!(
        "(self: Foo, x: number) -> number",
        to_string_type_id_to_string_options(
            fixture.require_type_string(&String::from("methodRef1")),
            &mut opts
        )
    );
    assert_eq!(
        "(self: Foo, x: number, y: string) -> number",
        to_string_type_id_to_string_options(
            fixture.require_type_string(&String::from("methodRef2")),
            &mut opts
        )
    );
    assert_eq!(
        "(a: number, b: string) -> string",
        to_string_type_id_to_string_options(
            fixture.require_type_string(&String::from("prop")),
            &mut opts
        )
    );
}
