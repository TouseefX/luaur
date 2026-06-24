#[cfg(test)]
#[test]
fn parser_tables_can_have_trailing_separator() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
        "local zero: number\n\
         local one: {x: number, y: string, }",
        &ParseOptions::default(),
    );
    assert!(!stat.is_null());
}
