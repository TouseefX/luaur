#[cfg(test)]
#[test]
fn parser_parsing_string_union_indexers() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from(r#"type foo = { ["bar" | "baz"]: number }"#);
    let options = ParseOptions::parse_options();
    let _result = fixture.parse_ex(&source, &options);
}
