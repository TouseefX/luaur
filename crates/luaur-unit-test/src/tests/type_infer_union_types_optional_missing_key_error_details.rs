//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_optional_missing_key_error_details() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = { x: number, y: number }
        type B = { x: number, y: number }
        type C = { x: number }
        type D = { x: number }

        function f(a: A | B | C | D)
            local y = a.y
            local z = a.z
        end

        function g(c: A | B | C | D | nil)
            local d = c.y
        end
    "#,
        ),
        None,
    );

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Key 'y' is missing from 'C', 'D' in the type 'A | B | C | D'",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!(
        "Type 'A | B | C | D' does not have key 'z'",
        to_string_type_error(&result.errors[1])
    );
    assert_eq!(
        "Value of type '(A | B | C | D)?' could be nil",
        to_string_type_error(&result.errors[2])
    );
    assert_eq!(
        "Key 'y' is missing from 'C', 'D' in the type 'A | B | C | D'",
        to_string_type_error(&result.errors[3])
    );
}
