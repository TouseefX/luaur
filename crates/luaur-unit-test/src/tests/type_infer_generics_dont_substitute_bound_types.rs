#[cfg(test)]
#[test]
fn type_infer_generics_dont_substitute_bound_types() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type T = { m: <a>(a) -> T }
        function f(t : T)
            local x: T = t.m(37)
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
