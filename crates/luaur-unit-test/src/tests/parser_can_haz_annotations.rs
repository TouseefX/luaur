#[cfg(test)]
#[test]
fn parser_can_haz_annotations() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
        "local foo: string = \"Hello Types!\"",
        &ParseOptions::default(),
    );
    assert!(!block.is_null());
}
