#[cfg(test)]
#[test]
fn parser_extra_token_in_consume_match_end() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = "\nif true then\n    return 12\nthen\nend\n";

    let result = fixture.try_parse(
        &source.to_string(),
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
        result.errors[0].get_message(),
        "Expected 'end' (to close 'then' at line 2), got 'then'"
    );
}
