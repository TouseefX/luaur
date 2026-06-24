#[cfg(test)]
#[test]
fn parser_parse_error_broken_comment() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let expected = alloc::string::String::from(
        "Expected identifier when parsing expression, got unfinished comment",
    );
    fixture.match_parse_error(
        &alloc::string::String::from("--[[unfinished work"),
        &expected,
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("--!strict\n--[[unfinished work"),
        &expected,
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("local x = 1 --[[unfinished work"),
        &expected,
        None,
    );
}
