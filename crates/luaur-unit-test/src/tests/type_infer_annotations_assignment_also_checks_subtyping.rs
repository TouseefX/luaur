//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_assignment_also_checks_subtyping() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(): number?
            return nil
        end
        local x: number = 1
        local y: number? = f()
        x = y
        y = x
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        Location {
            begin: Position {
                line: 6,
                column: 12,
            },
            end: Position {
                line: 6,
                column: 13,
            },
        },
        result.errors[0].location
    );
}
