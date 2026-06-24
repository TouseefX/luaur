#[cfg(test)]
#[test]
fn parser_type_assertion_expression() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let _stat = fixture.parse(
        r#"local a = something() :: any"#,
        &ParseOptions::parse_options(),
    );
    assert!(!_stat.is_null());
}
