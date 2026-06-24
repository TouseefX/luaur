#[cfg(test)]
#[test]
fn parser_parse_error_loop_control() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("break"),
        &alloc::string::String::from("break statement must be inside a loop"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("repeat local function a() break end until false"),
        &alloc::string::String::from("break statement must be inside a loop"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("continue"),
        &alloc::string::String::from("continue statement must be inside a loop"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("repeat local function a() continue end until false"),
        &alloc::string::String::from("continue statement must be inside a loop"),
        None,
    );
}
