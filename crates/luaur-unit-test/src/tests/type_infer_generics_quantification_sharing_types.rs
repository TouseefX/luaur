#[cfg(test)]
#[test]
fn type_infer_generics_quantification_sharing_types() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(x) return {5} end
        function g(x, y) return f(x) end
        local z1 = f(5)
        local z2 = g(true, "hi")
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let z1 = fixture.require_type_string(&String::from("z1"));
    let z2 = fixture.require_type_string(&String::from("z2"));
    assert_eq!(z1, z2);
}
