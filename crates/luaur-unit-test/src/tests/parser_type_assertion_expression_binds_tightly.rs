#[cfg(test)]
#[test]
fn parser_type_assertion_expression_binds_tightly() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_expr_binary::AstExprBinary;
    use luaur_ast::records::ast_expr_type_assertion::AstExprTypeAssertion;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_local::AstStatLocal;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
        "local a = one :: any + two :: any",
        &ParseOptions::default(),
    );
    let root = unsafe { &*stat };
    assert!(!root.body.data.is_null());

    let block = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatBlock>(
            root as *const _ as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!block.is_null());
    assert_eq!(1, unsafe { (*block).body.size });

    let local = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatLocal>(
            *(*block).body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!local.is_null());
    assert_eq!(1, unsafe { (*local).values.size });

    let bin = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprBinary>(
            *(*local).values.data.add(0) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!bin.is_null());

    assert!(!unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprTypeAssertion>(
            (*bin).left as *mut luaur_ast::records::ast_node::AstNode,
        )
        .is_null()
    });
    assert!(!unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprTypeAssertion>(
            (*bin).right as *mut luaur_ast::records::ast_node::AstNode,
        )
        .is_null()
    });
}
