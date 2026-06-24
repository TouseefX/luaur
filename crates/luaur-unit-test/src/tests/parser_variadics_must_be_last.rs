#[cfg(test)]
#[test]
fn parser_variadics_must_be_last() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("function foo(): (...number, string) end"),
        &alloc::string::String::from("Expected ')' (to close '(' at column 17), got ','"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("type Foo = (...number, string) -> (...string, number)"),
        &alloc::string::String::from("Expected ')' (to close '(' at column 12), got ','"),
        None,
    );
}
