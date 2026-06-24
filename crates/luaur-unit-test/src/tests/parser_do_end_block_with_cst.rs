#[cfg(test)]
#[test]
fn parser_do_end_block_with_cst() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::cst_stat_do::CstStatDo;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_ast::records::position::Position;
    use luaur_common::records::dense_hash_map::DenseHashMap;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from(
        "\n        do\n            local hello = \"world\"\n        end\n    ",
    );
    let mut parse_options = ParseOptions::parse_options();
    parse_options.store_cst_data = true;

    let result: ParseResult = fixture.parse_ex(&source, &parse_options);

    assert!(!result.root.is_null());

    let module_cst_node = result
        .cst_node_map
        .find(&(result.root as *mut AstStatBlock as *mut AstNode));
    assert!(module_cst_node.is_none());

    assert_eq!(1, unsafe { (*result.root).body.size });

    let do_block = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatBlock>(*(*result.root).body.data.add(0) as *mut AstNode)
    };
    assert!(!do_block.is_null());

    let do_block_cst_node = result.cst_node_map.find(&(do_block as *mut AstNode));
    assert!(!do_block_cst_node.is_none());

    let do_block_cst =
        unsafe { luaur_ast::rtti::cst_node_as::<CstStatDo>(*do_block_cst_node.unwrap()) };
    assert!(!do_block_cst.is_null());

    assert_eq!(Position::new(2, 12), unsafe {
        (*do_block_cst).stats_start_position
    });
    assert_eq!(Position::new(3, 8), unsafe { (*do_block_cst).end_position });
}
