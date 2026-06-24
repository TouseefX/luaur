#[cfg(test)]
#[test]
fn parser_parse_error_table_literal() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from(
            "function stringifyTable(t)\n    local foo = (name = t)\n    return foo\nend\n",
        ),
        &alloc::string::String::from(
            "Expected ')' (to close '(' at column 17), got '='; did you mean to use '{' when defining a table?",
        ),
        None,
    );
}
