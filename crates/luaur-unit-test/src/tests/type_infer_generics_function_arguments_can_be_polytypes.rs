#[cfg(test)]
#[test]
fn type_infer_generics_function_arguments_can_be_polytypes() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(g: <a>(a)->a)
            local x: number = g(37)
            local y: string = g("hi")
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
