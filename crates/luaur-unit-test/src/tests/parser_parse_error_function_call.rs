#[cfg(test)]
#[test]
fn parser_parse_error_function_call() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from(
            "function stringifyTable(t)\n    local foo = t:Parse 2\n    return foo\nend",
        ),
        &alloc::string::String::from(
            "Expected '(', '{' or <string> when parsing function call, got '2'",
        ),
        None,
    );
}
