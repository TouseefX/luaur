#[cfg(test)]
#[test]
fn type_infer_generics_local_vars_can_be_instantiated_polytypes() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function id(x) return x end
        print("This is bogus") -- TODO: CLI-39916
        local f: (number)->number = id
        local g: (string)->string = id
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
