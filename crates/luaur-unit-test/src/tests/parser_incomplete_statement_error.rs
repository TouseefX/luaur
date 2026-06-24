#[cfg(test)]
#[test]
fn parser_incomplete_statement_error() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("fiddlesticks"),
        &alloc::string::String::from(
            "Incomplete statement: expected assignment or a function call",
        ),
        None,
    );
}
