//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:982:type_infer_functions_calling_function_with_incorrect_argument_type_yields_errors_spanning_argument`
//! Source: `tests/TypeInfer.functions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.functions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.functions.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record Test (tests/NotNull.test.cpp)
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_functions_calling_function_with_incorrect_argument_type_yields_errors_spanning_argument

#[cfg(test)]
#[test]
fn type_infer_functions_calling_function_with_incorrect_argument_type_yields_errors_spanning_argument(
) {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function foo(a: number, b: string) end

        foo("Test", 123)
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
        Location {
            begin: Position {
                line: 3,
                column: 12
            },
            end: Position {
                line: 3,
                column: 18
            },
        },
        result.errors[0].location
    );
    let tm0 = unsafe { get_type_error::<TypeMismatch>(&result.errors[0]).as_ref() }
        .expect("expected TypeMismatch");
    assert_eq!("number", to_string_type_id(tm0.wanted_type));
    assert_eq!("string", to_string_type_id(tm0.given_type));

    assert_eq!(
        Location {
            begin: Position {
                line: 3,
                column: 20
            },
            end: Position {
                line: 3,
                column: 23
            },
        },
        result.errors[1].location
    );
    let tm1 = unsafe { get_type_error::<TypeMismatch>(&result.errors[1]).as_ref() }
        .expect("expected TypeMismatch");
    assert_eq!("string", to_string_type_id(tm1.wanted_type));
    assert_eq!("number", to_string_type_id(tm1.given_type));
}
