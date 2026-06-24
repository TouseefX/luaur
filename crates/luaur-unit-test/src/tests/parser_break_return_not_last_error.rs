#[cfg(test)]
#[test]
fn parser_break_return_not_last_error() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("return 0 print(5)"),
        &alloc::string::String::from("Expected <eof>, got 'print'"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("while true do break print(5) end"),
        &alloc::string::String::from("Expected 'end' (to close 'do' at column 12), got 'print'"),
        None,
    );
}
