//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:346:type_infer_functions_too_many_arguments_error_location`
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
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - calls -> function matches (Analysis/include/Luau/ControlFlow.h)
//!   - calls -> method StringWriter::identifier (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_functions_too_many_arguments_error_location

#[cfg(test)]
#[test]
fn type_infer_functions_too_many_arguments_error_location() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::records::count_mismatch::CountMismatch;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict

        function myfunction(a: number, b:number) end
        myfunction(1)

        function getmyfunction()
            return myfunction
        end
        getmyfunction()()
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
        Location {
            begin: Position { line: 4, column: 8 },
            end: Position {
                line: 4,
                column: 18
            }
        },
        result.errors[0].location
    );
    let acm0 = unsafe { get_type_error::<CountMismatch>(&result.errors[0]).as_ref() }
        .expect("expected CountMismatch");
    assert_eq!(2, acm0.expected());
    assert_eq!(1, acm0.actual());

    assert_eq!(
        Location {
            begin: Position { line: 9, column: 8 },
            end: Position {
                line: 9,
                column: 23
            }
        },
        result.errors[1].location
    );
    let acm1 = unsafe { get_type_error::<CountMismatch>(&result.errors[1]).as_ref() }
        .expect("expected CountMismatch");
    assert_eq!(2, acm1.expected());
    assert_eq!(0, acm1.actual());
}
