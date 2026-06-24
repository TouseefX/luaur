//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_table_intersection_write() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type X = { x: number }
        type XY = X & { y: number }

        function f(t: XY)
            t.x = 10
        end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type X = {}
        type XY = X & { x: number, y: number }

        function f(t: XY)
            t.x = 10
        end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type X = { x: number }
        type Y = { y: number }
        type XY = X & Y

        function f(t: XY)
            t.x = 10
        end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = { x: {y: number} }
        type B = { x: {y: number} }

        function f(t: A & B)
            t.x = { y = 4 }
            t.x.y = 40
        end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
