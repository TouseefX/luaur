//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_as_expr_is_bidirectional() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a = 55 :: number?
        local b = a :: number
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "number?",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
}
