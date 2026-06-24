#[cfg(test)]
#[test]
fn parser_function_type_matching_parenthesis() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("local a: <T>(number -> string"),
        &alloc::string::String::from("Expected ')' (to close '(' at column 13), got '->'"),
        None,
    );
}
