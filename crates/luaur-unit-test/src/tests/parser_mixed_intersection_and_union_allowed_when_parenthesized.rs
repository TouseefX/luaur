#[cfg(test)]
#[test]
fn parser_mixed_intersection_and_union_allowed_when_parenthesized() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.parse(
        "type A = (number & string) | boolean",
        &luaur_ast::records::parse_options::ParseOptions::parse_options(),
    );
}
