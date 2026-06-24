#[cfg(test)]
#[test]
fn parser_recover_type_index_name_keyword() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let source1 = alloc::string::String::from("local A\nlocal b : A.do\n");
    let options = ParseOptions::parse_options();
    let result1 = fixture.try_parse(&source1, &options);
    assert_eq!(1, result1.errors.len());

    let source2 = alloc::string::String::from("local A\nlocal b : A.do\ndo end\n");
    let result2 = fixture.try_parse(&source2, &options);
    assert_eq!(1, result2.errors.len());
}
