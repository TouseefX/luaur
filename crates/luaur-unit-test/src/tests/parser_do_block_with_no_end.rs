#[cfg(test)]
#[test]
fn parser_do_block_with_no_end() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("do\n");
    let options = ParseOptions::parse_options();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(1, result.errors.len());

    let stat0 = unsafe { (*result.root).body.data.add(0) };
    let stat0_block = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatBlock>(
            *stat0 as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!stat0_block.is_null());

    assert!(!unsafe { (*stat0_block).has_end });
}
