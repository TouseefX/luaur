//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:993:type_infer_builtins_string_format_report_all_type_errors_at_correct_positions`
//! Source: `tests/TypeInfer.builtins.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function format (tests/StringUtils.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - type_ref -> type_alias TypeErrorData (Analysis/include/Luau/Error.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_builtins_string_format_report_all_type_errors_at_correct_positions

#[cfg(test)]
#[test]
fn type_infer_builtins_string_format_report_all_type_errors_at_correct_positions() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        ("%s%d%s"):format(1, "hello", true)
        string.format("%s%d%s", 1, "hello", true)
    "#,
        ),
        None,
    );

    assert_eq!(6, result.errors.len(), "{:?}", result.errors);

    let expected = [
        (
            Location {
                begin: Position {
                    line: 1,
                    column: 26,
                },
                end: Position {
                    line: 1,
                    column: 27,
                },
            },
            "Expected this to be 'string', but got 'number'",
        ),
        (
            Location {
                begin: Position {
                    line: 1,
                    column: 29,
                },
                end: Position {
                    line: 1,
                    column: 36,
                },
            },
            "Expected this to be 'number', but got 'string'",
        ),
        (
            Location {
                begin: Position {
                    line: 1,
                    column: 38,
                },
                end: Position {
                    line: 1,
                    column: 42,
                },
            },
            "Expected this to be 'string', but got 'boolean'",
        ),
        (
            Location {
                begin: Position {
                    line: 2,
                    column: 32,
                },
                end: Position {
                    line: 2,
                    column: 33,
                },
            },
            "Expected this to be 'string', but got 'number'",
        ),
        (
            Location {
                begin: Position {
                    line: 2,
                    column: 35,
                },
                end: Position {
                    line: 2,
                    column: 42,
                },
            },
            "Expected this to be 'number', but got 'string'",
        ),
        (
            Location {
                begin: Position {
                    line: 2,
                    column: 44,
                },
                end: Position {
                    line: 2,
                    column: 48,
                },
            },
            "Expected this to be 'string', but got 'boolean'",
        ),
    ];

    for (index, (location, message)) in expected.into_iter().enumerate() {
        assert_eq!(location, result.errors[index].location);
        assert_eq!(message, to_string_type_error(&result.errors[index]));
    }
}
