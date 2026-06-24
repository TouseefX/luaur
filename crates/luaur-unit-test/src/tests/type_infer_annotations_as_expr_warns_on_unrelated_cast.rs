//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_as_expr_warns_on_unrelated_cast() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a = 55 :: string
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Cannot cast 'number' into 'string' because the types are unrelated",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!(
        "string",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
}
