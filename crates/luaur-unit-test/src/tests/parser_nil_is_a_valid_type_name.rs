#[cfg(test)]
#[test]
fn parser_nil_is_a_valid_type_name() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("local n: nil", &ParseOptions::default());
    assert!(!stat.is_null());
}
