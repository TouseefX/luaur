#[cfg(test)]
#[test]
fn type_infer_generics_check_generic_function() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function id<a>(x:a): a
            return x
        end
        local x: string = id("hi")
        local y: number = id(37)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "string",
        to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
    assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("y")))
    );
}
