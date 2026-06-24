#[cfg(test)]
#[test]
fn parser_basic_parse() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let _stat = fixture.parse(r#"print("Hello World!")"#, &ParseOptions::parse_options());
    assert!(!_stat.is_null());
}
