#[cfg(test)]
#[test]
fn parser_explicit_type_instantiation_errors() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("local a = x:a<<T>>"),
        &alloc::string::String::from(
            "Expected '(', '{' or <string> when parsing function call, got <eof>",
        ),
        None,
    );
}
