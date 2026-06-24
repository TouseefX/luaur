#[cfg(test)]
#[test]
fn parser_annotations_can_be_tables() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
        "local zero: number\n\
         local one: {x: number, y: string}",
        &ParseOptions::default(),
    );
    let root = unsafe { &*stat };
    assert!(!root.body.data.is_null());
}
