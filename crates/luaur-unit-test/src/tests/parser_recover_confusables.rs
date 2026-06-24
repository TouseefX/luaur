#[cfg(test)]
#[test]
fn parser_recover_confusables() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();

    // Binary
    fixture.match_parse_error(
        &alloc::string::String::from("local a = 4 != 10"),
        &alloc::string::String::from("Unexpected '!='; did you mean '~='?"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("local a = true && false"),
        &alloc::string::String::from("Unexpected '&&'; did you mean 'and'?"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("local a = false || true"),
        &alloc::string::String::from("Unexpected '||'; did you mean 'or'?"),
        None,
    );

    // Unary
    fixture.match_parse_error(
        &alloc::string::String::from("local a = !false"),
        &alloc::string::String::from("Unexpected '!'; did you mean 'not'?"),
        None,
    );

    // Check that separate tokens are not considered as a single one
    fixture.match_parse_error(
        &alloc::string::String::from("local a = 4 ! = 10"),
        &alloc::string::String::from("Expected identifier when parsing expression, got '!'"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("local a = true & & false"),
        &alloc::string::String::from("Expected identifier when parsing expression, got '&'"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("local a = false | | true"),
        &alloc::string::String::from("Expected identifier when parsing expression, got '|'"),
        None,
    );
}
