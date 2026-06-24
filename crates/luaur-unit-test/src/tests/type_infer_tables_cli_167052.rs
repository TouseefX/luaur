#[cfg(test)]
#[test]
fn type_infer_tables_cli_167052() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local Children = newproxy()
        local Macro: { [ string | typeof(Children) ]: true } = {
            ["_exec"] = true;
            ["_run"] = true;
            ["_init"] = true;
            ["_base"] = true;
            ["Class"] = true;
            ["_count"] = true;
            [Children] = true;
        }
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
