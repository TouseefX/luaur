#[cfg(test)]
#[test]
fn parser_last_line_does_not_have_to_be_blank() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let source = "-- print('hello')";
    let options = ParseOptions::parse_options();
    let _result = fixture.parse(source, &options);
}
