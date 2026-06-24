//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_occurs_check_on_cyclic_intersection_type() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::occurs_check_failed::OccursCheckFailed;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type T = T & T
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(
        type_error_data_ref::<OccursCheckFailed>(&result.errors[0]).is_some(),
        "expected OccursCheckFailed: {:?}",
        result.errors[0]
    );
}
