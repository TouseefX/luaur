#[cfg(test)]
#[test]
fn parser_mixed_leading_intersection_and_union_not_allowed() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("type A = & number | string | boolean"),
        &alloc::string::String::from(
            "Mixing union and intersection types is not allowed; consider wrapping in parentheses.",
        ),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("type A = | number & string & boolean"),
        &alloc::string::String::from(
            "Mixing union and intersection types is not allowed; consider wrapping in parentheses.",
        ),
        None,
    );
}
