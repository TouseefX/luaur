//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_multi_assign_checks_against_annotations() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a: number, b: string = 1, "two"
        a, b = "one", 2
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        Location {
            begin: Position {
                line: 2,
                column: 15,
            },
            end: Position {
                line: 2,
                column: 20,
            },
        },
        result.errors[0].location
    );
    assert_eq!(
        Location {
            begin: Position {
                line: 2,
                column: 22,
            },
            end: Position {
                line: 2,
                column: 23,
            },
        },
        result.errors[1].location
    );
}
