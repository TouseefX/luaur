#[cfg(test)]
#[test]
fn parser_functions_can_return_0_values() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let block = fixture.parse("local f: (number) -> ()", &ParseOptions::default());
    assert!(!block.is_null());
}
