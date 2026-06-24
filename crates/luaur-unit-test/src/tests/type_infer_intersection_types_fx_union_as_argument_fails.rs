//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_fx_union_as_argument_fails() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = (number) -> (string)
        type B = (string) -> (number)
        type C = (A) -> (number)

        local function foo(f: A | B, g: C)
            return g(f)
        end
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty());
}
