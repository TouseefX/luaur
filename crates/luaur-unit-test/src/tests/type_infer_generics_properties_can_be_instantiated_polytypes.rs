#[cfg(test)]
#[test]
fn type_infer_generics_properties_can_be_instantiated_polytypes() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t: { m: (number)->number } = { m = function(x:number) return x+1 end }
        local function id<a>(x:a):a return x end
        t.m = id
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
