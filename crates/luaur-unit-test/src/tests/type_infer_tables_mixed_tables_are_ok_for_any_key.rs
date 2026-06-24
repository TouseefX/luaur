#[cfg(test)]
#[test]
fn type_infer_tables_mixed_tables_are_ok_for_any_key() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local foo: { [any]: unknown } = {
            Key = "sorry",
            "A",
            "B",
        }
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
