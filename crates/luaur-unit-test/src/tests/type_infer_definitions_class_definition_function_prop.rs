//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:267:type_infer_definitions_class_definition_function_prop`
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
//!   - translates_to -> rust_item type_infer_definitions_class_definition_function_prop

#[cfg(test)]
#[test]
fn type_infer_definitions_class_definition_function_prop() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
        &String::from(
            r#"
        declare class Foo
            X: (number) -> string
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
        local prop = x.X
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "(number) -> string",
        to_string_type_id(fixture.require_type_string(&String::from("prop")))
    );
}
