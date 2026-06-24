#[cfg(test)]
#[test]
fn parser_parse_interpolated_string_with_lookahead_involved() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("local x = `{ {y} }`");
    let options = ParseOptions::parse_options();
    let result = fixture.try_parse(&source, &options);
    assert!(result.errors.is_empty());
}
