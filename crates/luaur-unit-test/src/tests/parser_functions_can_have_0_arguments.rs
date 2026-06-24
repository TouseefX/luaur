#[cfg(test)]
#[test]
fn parser_functions_can_have_0_arguments() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("local f: () -> number", &ParseOptions::default());
    assert!(!stat.is_null());
}
