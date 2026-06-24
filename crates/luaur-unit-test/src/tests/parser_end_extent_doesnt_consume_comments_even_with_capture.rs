#[cfg(test)]
#[test]
fn parser_end_extent_doesnt_consume_comments_even_with_capture() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_stat::AstStat;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::position::Position;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from(
        "\n        type F = number\n        --comment\n        print('hello')\n    ",
    );
    let mut opts = ParseOptions::parse_options();
    opts.capture_comments = true;

    let block = fix.parse(&code, &opts);

    unsafe {
        assert_eq!(2, (*block).body.size);
        let first_stat: *mut AstStat = *((*block).body.data.add(0));
        let expected_end = Position::new(1, 23);
        assert_eq!(expected_end, (*first_stat).base.location.end);
    }
}
