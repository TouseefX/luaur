#[cfg(test)]
#[test]
fn parser_extra_table_indexer_recovery() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let parse_result = fixture.try_parse(
        &String::from("local a : { [string] : number, [number] : string, count: number }"),
        &ParseOptions::default(),
    );
    assert_eq!(parse_result.errors.len(), 1);
}
