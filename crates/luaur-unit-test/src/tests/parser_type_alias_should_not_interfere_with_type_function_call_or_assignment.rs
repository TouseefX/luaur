#[cfg(test)]
#[test]
fn parser_type_alias_should_not_interfere_with_type_function_call_or_assignment() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_expr_call::AstExprCall;
    use luaur_ast::records::ast_stat_assign::AstStatAssign;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_expr::AstStatExpr;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::rtti::ast_node_as;
    use luaur_ast::rtti::ast_node_is;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
        "type(\"a\")\n\
         type = nil",
        &ParseOptions::default(),
    );
    assert!(!block.is_null());

    let block_ref = unsafe { &*block };
    assert!(block_ref.body.size > 0);

    let first_stat = unsafe { *block_ref.body.data.add(0) };
    assert!(!first_stat.is_null());

    let stat_expr = unsafe {
        ast_node_as::<AstStatExpr>(first_stat as *mut luaur_ast::records::ast_node::AstNode)
    };
    assert!(!stat_expr.is_null());

    let expr_call = unsafe {
        ast_node_as::<AstExprCall>((*stat_expr).expr as *mut luaur_ast::records::ast_node::AstNode)
    };
    assert!(!expr_call.is_null());

    let second_stat = unsafe { *block_ref.body.data.add(1) };
    assert!(!second_stat.is_null());

    let is_assign = unsafe {
        ast_node_is::<AstStatAssign>(second_stat as *const luaur_ast::records::ast_node::AstNode)
    };
    assert!(is_assign);
}
