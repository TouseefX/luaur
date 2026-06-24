#[cfg(test)]
#[test]
fn parser_explicit_type_instantiation_expression() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("local x = f<<T, U>>", &ParseOptions::default());
    assert!(!stat.is_null());
}
