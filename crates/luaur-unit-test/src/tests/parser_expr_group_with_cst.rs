#[cfg(test)]
#[test]
fn parser_expr_group_with_cst() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::records::ast_expr_group::AstExprGroup;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat_local::AstStatLocal;
    use luaur_ast::records::cst_expr_group::CstExprGroup;
    use luaur_ast::records::cst_node::CstNode;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_ast::records::position::Position;
    use luaur_ast::rtti::ast_node_as;
    use luaur_ast::rtti::cst_node_as;

    let _scoped_flag = ScopedFastFlag::new(&luaur_common::FFlag::LuauCstExprGroup, true);
    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("\n        local a = (1 + 2)\n    ");
    let mut parse_options = ParseOptions::parse_options();
    parse_options.store_cst_data = true;

    let result: ParseResult = fixture.parse_ex(&source, &parse_options);

    assert!(!result.root.is_null());
    assert_eq!(1, unsafe { (*result.root).body.size });

    let local_stmt = unsafe {
        let stmt_ptr = (*result.root).body.data.add(0);
        let stmt_node = &**stmt_ptr;
        ast_node_as::<AstStatLocal>(
            stmt_node as *const luaur_ast::records::ast_stat::AstStat as *mut AstNode,
        )
    };
    assert!(!local_stmt.is_null());

    assert_eq!(1, unsafe { (*local_stmt).values.size });

    let group_expr = unsafe {
        let expr_ptr = (*local_stmt).values.data.add(0);
        let expr_node = &**expr_ptr;
        ast_node_as::<AstExprGroup>(
            expr_node as *const luaur_ast::records::ast_expr::AstExpr as *mut AstNode,
        )
    };
    assert!(!group_expr.is_null());

    let base_cst_node = result
        .cst_node_map
        .find(&(group_expr as *mut AstNode))
        .copied()
        .unwrap_or(core::ptr::null_mut());
    assert!(!base_cst_node.is_null());

    let cst_node = unsafe {
        let node_ref = &*base_cst_node;
        cst_node_as::<CstExprGroup>(node_ref as *const CstNode as *mut CstNode)
    };
    assert!(!cst_node.is_null());

    let expected_close_position = Position::new(1, 24);
    assert_eq!(expected_close_position, unsafe {
        (*cst_node).close_position
    });
}
