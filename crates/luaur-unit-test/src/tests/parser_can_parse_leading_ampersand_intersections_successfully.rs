#[cfg(test)]
#[test]
fn parser_can_parse_leading_ampersand_intersections_successfully() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.parse(
        "type A = & { string } & { number }",
        &luaur_ast::records::parse_options::ParseOptions::parse_options(),
    );
}
