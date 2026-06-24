#[cfg(test)]
#[test]
fn type_infer_generics_check_recursive_generic_function() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function id<a>(x:a):a
            local y: string = id("hi")
            local z: number = id(37)
            return x
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
