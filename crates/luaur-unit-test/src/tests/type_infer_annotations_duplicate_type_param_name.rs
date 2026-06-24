//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_duplicate_type_param_name() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::duplicate_generic_parameter::DuplicateGenericParameter;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Oopsies<T, T> = {a: T, b: T}
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let dgp = type_error_data_ref::<DuplicateGenericParameter>(&result.errors[0])
        .expect("expected DuplicateGenericParameter");
    assert_eq!("T", dgp.parameterName());
}
