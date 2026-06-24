#[cfg(test)]
#[test]
fn parser_can_parse_leading_bar_unions_successfully() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("type A = | \"Hello\" | \"World\"");
    let options = ParseOptions::parse_options();
    let _result = fixture.parse_ex(&source, &options);
}
