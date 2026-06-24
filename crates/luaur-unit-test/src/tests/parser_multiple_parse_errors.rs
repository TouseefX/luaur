#[cfg(test)]
#[test]
fn parser_multiple_parse_errors() {
    use crate::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let source = alloc::string::String::from("local a = 3 * (\nreturn a +\n");

    let result = fix.try_parse(
        &source,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    assert_eq!(2, result.errors.len());
}
