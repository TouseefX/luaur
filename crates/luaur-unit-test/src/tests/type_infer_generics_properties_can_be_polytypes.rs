#[cfg(test)]
#[test]
fn type_infer_generics_properties_can_be_polytypes() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t = {}
        t.m = function<a>(x: a):a return x end
        local x: string = t.m("hi")
        local y: number = t.m(37)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
