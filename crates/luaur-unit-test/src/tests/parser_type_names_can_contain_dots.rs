#[cfg(test)]
#[test]
fn parser_type_names_can_contain_dots() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let block = fixture.parse("local foo: SomeModule.CoolType", &ParseOptions::default());
    assert!(!block.is_null());
}
