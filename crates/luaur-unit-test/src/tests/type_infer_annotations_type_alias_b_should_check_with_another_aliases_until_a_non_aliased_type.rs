//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_type_alias_b_should_check_with_another_aliases_until_a_non_aliased_type()
{
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = number
        type B = A
        local b: B = 10
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
