#[cfg(test)]
#[test]
fn type_infer_generics_check_generic_typepack_function() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function id<a...>(...: a...): (a...) return ... end
        local x: string, y: boolean = id("hi", true)
        local z: number = id(37)
        id()
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
