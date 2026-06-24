#[cfg(test)]
#[test]
fn parser_function_type_annotation() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("local f: (number, string) -> nil", &ParseOptions::default());
    assert!(!stat.is_null());
}
