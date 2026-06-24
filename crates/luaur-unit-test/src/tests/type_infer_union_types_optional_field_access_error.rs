//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_optional_field_access_error() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = { x: number }
        function f(b: A?)
            local c = b.x
            local d = b.y
        end
    "#,
        ),
        None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Value of type 'A?' could be nil",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!(
        "Value of type 'A?' could be nil",
        to_string_type_error(&result.errors[1])
    );
    assert_eq!(
        "Key 'y' not found in table 'A'",
        to_string_type_error(&result.errors[2])
    );
}
