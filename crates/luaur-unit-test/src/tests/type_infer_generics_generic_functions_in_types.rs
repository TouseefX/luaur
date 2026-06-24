#[cfg(test)]
#[test]
fn type_infer_generics_generic_functions_in_types() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type T = { id: <a>(a) -> a }
        local x: T = { id = function<a>(x:a):a return x end }
        local y: string = x.id("hi")
        local z: number = x.id(37)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
