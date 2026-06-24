#[cfg(test)]
#[test]
fn parser_extra_token_in_consume() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = "\nfunction test + (a, f) return a + f end\nreturn test(2, 3)\n";

    let result = fixture.try_parse(
        &source.to_string(),
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
        result.errors[0].get_message(),
        "Expected '(' when parsing function, got '+'"
    );
}
