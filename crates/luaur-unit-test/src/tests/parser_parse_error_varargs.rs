#[cfg(test)]
#[test]
fn parser_parse_error_varargs() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("function add(x, y) return ... end"),
        &alloc::string::String::from("Cannot use '...' outside of a vararg function"),
        None,
    );
}
