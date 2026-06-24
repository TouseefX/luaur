#[cfg(test)]
#[test]
fn parser_for_loop_with_single_var_has_comma_positions_of_size_zero() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_stat_for_in::AstStatForIn;
    use luaur_ast::records::cst_stat_for_in::CstStatForIn;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_ast::rtti::ast_node_as;
    use luaur_ast::rtti::cst_node_as;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("for value in tbl do\nend");
    let mut options = ParseOptions::parse_options();
    options.store_cst_data = true;
    let result = fixture.parse_ex(&source, &options);

    assert!(!result.root.is_null());
    assert_eq!(1, unsafe { (*result.root).body.size });

    let for_loop = unsafe { *(*result.root).body.data.add(0) };
    let for_loop = unsafe {
        ast_node_as::<AstStatForIn>(for_loop as *mut luaur_ast::records::ast_node::AstNode)
    };
    assert!(!for_loop.is_null());

    let base_cst_node = result
        .cst_node_map
        .find(&(for_loop as *mut luaur_ast::records::ast_node::AstNode));
    assert!(base_cst_node.is_some());

    let cst_node = unsafe { cst_node_as::<CstStatForIn>(*base_cst_node.unwrap()) };
    assert!(!cst_node.is_null());

    assert_eq!(0, unsafe { (*cst_node).vars_comma_positions.size });
}
