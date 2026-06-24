//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_mismatched_generic_pack_type_param() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type T<A...> = (A) -> ()
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Variadic type parameter 'A...' is used as a regular generic type; consider changing 'A...' to 'A' in the generic argument list",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!(
        Location {
            begin: Position {
                line: 1,
                column: 24,
            },
            end: Position {
                line: 1,
                column: 25,
            },
        },
        result.errors[0].location
    );
}
