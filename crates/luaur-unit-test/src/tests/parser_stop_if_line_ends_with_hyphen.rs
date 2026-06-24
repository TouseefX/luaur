#[cfg(test)]
#[test]
fn parser_stop_if_line_ends_with_hyphen() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    // C++ CHECK_THROWS_AS(parse("   -"), std::exception) — parsing must fail.
    // `fixture.parse` PANICS on errors (emulating the throw), so use try_parse and
    // assert the error instead of letting the panic escape the test.
    let result = fixture.try_parse(
        &alloc::string::String::from("   -"),
        &luaur_ast::records::parse_options::ParseOptions::parse_options(),
    );
    assert!(
        !result.errors.is_empty(),
        "Expected a parse error for a trailing hyphen"
    );
}
