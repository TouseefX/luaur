#[cfg(test)]
#[test]
fn parser_statement_error_recovery_unexpected() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("+");

    let result = fixture.try_parse(
        &source,
        &luaur_ast::records::parse_options::ParseOptions::parse_options(),
    );

    assert_eq!(1, result.errors.len());
}
