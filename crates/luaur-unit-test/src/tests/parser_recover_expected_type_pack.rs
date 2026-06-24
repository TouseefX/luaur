#[cfg(test)]
#[test]
fn parser_recover_expected_type_pack() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("type Y<T..., U = T...> = (T...) -> U...\n");
    let options = ParseOptions::parse_options();
    let result = fixture.try_parse(&source, &options);
    assert_eq!(1, result.errors.len());
}
