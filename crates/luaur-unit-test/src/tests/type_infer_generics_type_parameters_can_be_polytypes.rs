#[cfg(test)]
#[test]
fn type_infer_generics_type_parameters_can_be_polytypes() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function id<a>(x:a):a return x end
        local f: <a>(a)->a = id(id)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
