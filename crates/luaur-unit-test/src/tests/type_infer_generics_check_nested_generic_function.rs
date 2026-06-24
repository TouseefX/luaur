#[cfg(test)]
#[test]
fn type_infer_generics_check_nested_generic_function() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f()
            local function id<a>(x:a): a
                return x
            end
            local x: string = id("hi")
            local y: number = id(37)
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
