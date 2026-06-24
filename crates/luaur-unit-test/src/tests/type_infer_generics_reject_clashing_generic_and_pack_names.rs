#[cfg(test)]
#[test]
fn type_infer_generics_reject_clashing_generic_and_pack_names() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::duplicate_generic_parameter::DuplicateGenericParameter;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f<a, a...>() end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = type_error_data_ref::<DuplicateGenericParameter>(&result.errors[0])
        .expect("expected DuplicateGenericParameter");
    assert_eq!("a", err.parameterName());
}
