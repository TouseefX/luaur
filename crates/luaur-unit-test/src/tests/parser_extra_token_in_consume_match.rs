#[cfg(test)]
#[test]
fn parser_extra_token_in_consume_match() {
    use crate::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code =
        alloc::string::String::from("function test(a, f+) return a + f end\nreturn test(2, 3)\n");

    let result = fix.try_parse(
        &code,
        &luaur_ast::records::parse_options::ParseOptions::parse_options(),
    );

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
        &*result.errors[0].get_message(),
        "Expected ')' (to close '(' at column 14), got '+'"
    );
}
