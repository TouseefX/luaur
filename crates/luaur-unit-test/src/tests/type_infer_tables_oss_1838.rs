#[cfg(test)]
#[test]
fn type_infer_tables_oss_1838() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local myTable = {}
        myTable.foo = {}
        myTable.foo.bar = {}
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
