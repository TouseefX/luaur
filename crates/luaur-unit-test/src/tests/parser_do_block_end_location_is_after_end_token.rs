#[cfg(test)]
#[test]
fn parser_do_block_end_location_is_after_end_token() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
        "\n        do\n            local x = 1\n        end\n    ",
        &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    let root = unsafe { &*stat };
    assert_eq!(root.body.size, 1);

    let block = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatBlock>(
            *root.body.data as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!block.is_null());

    let expected_location = Location::new(
        luaur_ast::records::position::Position::new(1, 8),
        luaur_ast::records::position::Position::new(3, 11),
    );
    assert_eq!(unsafe { (*block).base.base.location }, expected_location);
}
