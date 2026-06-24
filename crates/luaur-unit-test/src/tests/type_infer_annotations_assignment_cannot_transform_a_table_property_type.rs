//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_assignment_cannot_transform_a_table_property_type() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a = {x=0}
        a.x = "one"
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        Location {
            begin: Position {
                line: 2,
                column: 14,
            },
            end: Position {
                line: 2,
                column: 19,
            },
        },
        result.errors[0].location
    );
}
