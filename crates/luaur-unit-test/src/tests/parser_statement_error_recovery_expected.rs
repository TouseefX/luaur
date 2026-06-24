#[cfg(test)]
#[test]
fn parser_statement_error_recovery_expected() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from("function a(a, b) return a + b end\nsome\na(2, 5)");
    let opts = ParseOptions::parse_options();

    let result = fix.try_parse(&code, &opts);

    assert_eq!(result.errors.len(), 1);
}
