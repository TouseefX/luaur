#[cfg(test)]
#[test]
fn parser_recover_index_name_keyword() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let source1 = alloc::string::String::from("local b\nlocal a = b.do\n");
    let options = ParseOptions::parse_options();
    let result1 = fixture.try_parse(&source1, &options);
    assert_eq!(1, result1.errors.len());

    let source2 = alloc::string::String::from("local b\nlocal a = b.\ndo end\n");
    let result2 = fixture.try_parse(&source2, &options);
    assert_eq!(1, result2.errors.len());
}
