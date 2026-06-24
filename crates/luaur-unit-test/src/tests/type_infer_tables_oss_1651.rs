#[cfg(test)]
#[test]
fn type_infer_tables_oss_1651() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        local MyModule = {}
        MyModule._isEnabled = true :: boolean

        assert(MyModule._isEnabled, `type solver`)
        MyModule._isEnabled = false
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
