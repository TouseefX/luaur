#[cfg(test)]
#[test]
fn parser_recover_self_call_keyword() {
    use crate::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let source1 = alloc::string::String::from("local b\nlocal a = b:do\n    ");
    let result1 = fix.try_parse(
        &source1,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );
    assert_eq!(2, result1.errors.len());

    let mut fix = Fixture::default();
    let source2 = alloc::string::String::from("local b\nlocal a = b:\ndo end\n    ");
    let result2 = fix.try_parse(
        &source2,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );
    assert_eq!(2, result2.errors.len());
}
