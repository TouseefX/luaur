#[cfg(test)]
#[test]
fn parser_explicit_type_instantiation_empty_list() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("f<<>>()", &ParseOptions::default());
    assert!(!stat.is_null());
}
