#[cfg(test)]
#[test]
fn parser_parse_interpolated_string_call_without_parens() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = alloc::string::String::from("_ = print `{42}`");
    let expected =
        alloc::string::String::from("Expected identifier when parsing expression, got `{");

    fixture.match_parse_error(&source, &expected, None);
}
