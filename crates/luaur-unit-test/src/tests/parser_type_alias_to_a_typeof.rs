#[cfg(test)]
#[test]
fn parser_type_alias_to_a_typeof() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
        // C++ R"(\n        type A = typeof(1)\n    )" — leading newline + 8-space
        // indent put the alias at {1,8}; the port dropped both.
        "\n        type A = typeof(1)\n    ",
        &ParseOptions::default(),
    );
    assert!(!block.is_null());

    let root = unsafe { &*block };
    assert!(root.body.size > 0);

    let type_alias_stat = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatTypeAlias>(
            *root.body.data as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!type_alias_stat.is_null());

    let expected_location = Location::new(
        luaur_ast::records::position::Position::new(1, 8),
        luaur_ast::records::position::Position::new(1, 26),
    );
    assert_eq!(
        unsafe { (*type_alias_stat).base.base.location },
        expected_location
    );
}
