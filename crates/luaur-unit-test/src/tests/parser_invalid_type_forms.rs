#[cfg(test)]
#[test]
fn parser_invalid_type_forms() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("type A = (b: number)"),
        &alloc::string::String::from("Expected '->' when parsing function type, got <eof>"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from(
            "type P<T...> = () -> T... type B = P<(x: number, y: string)>",
        ),
        &alloc::string::String::from("Expected '->' when parsing function type, got '>'"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("type F<T... = (a: string)> = (T...) -> ()"),
        &alloc::string::String::from("Expected '->' when parsing function type, got '>'"),
        None,
    );
}
