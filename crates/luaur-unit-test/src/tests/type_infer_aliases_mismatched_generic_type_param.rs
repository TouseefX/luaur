//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_mismatched_generic_type_param() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type T<A> = (A...) -> ()
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Generic type 'A' is used as a variadic type parameter; consider changing 'A' to 'A...' in the generic argument list",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!(
        Location {
            begin: Position {
                line: 1,
                column: 21,
            },
            end: Position {
                line: 1,
                column: 25,
            },
        },
        result.errors[0].location
    );
}
