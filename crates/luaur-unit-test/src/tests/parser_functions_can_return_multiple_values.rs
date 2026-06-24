#[cfg(test)]
#[test]
fn parser_functions_can_return_multiple_values() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
        "local f: (number) -> (number, number)",
        &ParseOptions::default(),
    );
    assert!(!stat.is_null());
}
