#[cfg(test)]
#[test]
fn type_infer_generics_calling_self_generic_methods() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local x = {}
        function x:id(x) return x end
        function x:f()
            local x: string = self:id("hi")
            local y: number = self:id(37)
        end
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "expected errors");
}
