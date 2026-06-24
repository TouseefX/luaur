#[cfg(test)]
#[test]
fn parser_end_extent_doesnt_consume_comments() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let source = alloc::string::String::from(
        "\n        type F = number\n        --comment\n        print('hello')\n    ",
    );
    let block = fixture.parse(&source, &ParseOptions::parse_options());
    unsafe {
        let body = (*block).body;
        assert_eq!(2, body.size as usize);
        let first_stat = *body.data.add(0);
        let first_loc = (*first_stat).base.location;
        assert_eq!(
            luaur_ast::records::position::Position::new(1, 23),
            first_loc.end
        );
    }
}
