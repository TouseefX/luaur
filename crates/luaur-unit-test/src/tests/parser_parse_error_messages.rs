#[cfg(test)]
#[test]
fn parser_parse_error_messages() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("\n        local a: (number, number) -> (string\n    "),
        &alloc::string::String::from("Expected ')' (to close '(' at line 2), got <eof>"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from(
            "\n        local a: (number, number) -> (\n            string\n    ",
        ),
        &alloc::string::String::from("Expected ')' (to close '(' at line 2), got <eof>"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("\n        local a: (number, number)\n    "),
        &alloc::string::String::from("Expected '->' when parsing function type, got <eof>"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("\n        local a: (number, number\n    "),
        &alloc::string::String::from("Expected ')' (to close '(' at line 2), got <eof>"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("\n        local a: {foo: string,\n    "),
        &alloc::string::String::from("Expected identifier when parsing table field, got <eof>"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("\n        local a: {foo: string\n    "),
        &alloc::string::String::from("Expected '}' (to close '{' at line 2), got <eof>"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from(
            "\n        local a: { [string]: number, [number]: string }\n    ",
        ),
        &alloc::string::String::from("Cannot have more than one table indexer"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("\n        type T = <a>foo\n    "),
        &alloc::string::String::from("Expected '(' when parsing function parameters, got 'foo'"),
        None,
    );
}
