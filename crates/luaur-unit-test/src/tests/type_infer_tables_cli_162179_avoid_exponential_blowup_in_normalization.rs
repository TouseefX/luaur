#[cfg(test)]
#[test]
fn type_infer_tables_cli_162179_avoid_exponential_blowup_in_normalization() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut entries = String::new();
    for _ in 0..100 {
        entries.push_str("\"foo\",");
    }

    let source = String::from(
        r#"
        local res = { "#,
    ) + &entries
        + r#" }

        local function check(index: number)
            if res[index] == "foo" then
                print("found a foo!")
            end
        end
    "#;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture
        .base
        .check_string_optional_frontend_options(&source, None);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
