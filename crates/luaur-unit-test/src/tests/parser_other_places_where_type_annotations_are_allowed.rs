#[cfg(test)]
#[test]
fn parser_other_places_where_type_annotations_are_allowed() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
        "for i: number = 0, 50 do end\n\
         for i: number, s: string in expr() do end",
        &ParseOptions::default(),
    );
    assert!(!stat.is_null());
}
