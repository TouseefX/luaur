#[cfg(test)]
#[test]
fn parser_tables_should_have_an_indexer_and_keys() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    // Faithful to the C++ R-string; the port injected stray `"` at each line start
    // (turning it into a malformed string literal).
    let stat = fixture.parse(
        "\n        local t: {\n            [string]: number,\n            f: () -> nil\n        }\n    ",
        &ParseOptions::default(),
    );
    assert!(!stat.is_null());
}
