#[cfg(test)]
#[test]
fn parser_parse_numbers_decimal() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_expr::AstExpr;
    use luaur_ast::records::ast_expr_constant_integer::AstExprConstantInteger;
    use luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat::AstStat;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_return::AstStatReturn;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::rtti::AstNodeClass;
    use luaur_common::FFlag;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
        "return 1, .5, 1.5, 1e-5, 1.5e-5, 12_345.1_25",
        &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    let block = unsafe { &*stat };
    assert!(!block.body.data.is_null());

    let first_stat = unsafe { *block.body.data.add(0) };
    let return_stat =
        unsafe { luaur_ast::rtti::ast_node_as::<AstStatReturn>(first_stat as *mut AstNode) };
    assert!(!return_stat.is_null());

    let return_stat = unsafe { &*return_stat };
    assert_eq!(return_stat.list.size, 6);

    let expr0 = unsafe { &*(*return_stat.list.data.add(0)) };
    let const_num0 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprConstantNumber>(
            expr0 as *const AstExpr as *mut AstNode,
        )
    };
    assert!(!const_num0.is_null());
    assert_eq!(unsafe { &*const_num0 }.value, 1.0);

    let expr1 = unsafe { &*(*return_stat.list.data.add(1)) };
    let const_num1 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprConstantNumber>(
            expr1 as *const AstExpr as *mut AstNode,
        )
    };
    assert!(!const_num1.is_null());
    assert_eq!(unsafe { &*const_num1 }.value, 0.5);

    let expr2 = unsafe { &*(*return_stat.list.data.add(2)) };
    let const_num2 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprConstantNumber>(
            expr2 as *const AstExpr as *mut AstNode,
        )
    };
    assert!(!const_num2.is_null());
    assert_eq!(unsafe { &*const_num2 }.value, 1.5);

    let expr3 = unsafe { &*(*return_stat.list.data.add(3)) };
    let const_num3 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprConstantNumber>(
            expr3 as *const AstExpr as *mut AstNode,
        )
    };
    assert!(!const_num3.is_null());
    assert_eq!(unsafe { &*const_num3 }.value, 1.0e-5);

    let expr4 = unsafe { &*(*return_stat.list.data.add(4)) };
    let const_num4 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprConstantNumber>(
            expr4 as *const AstExpr as *mut AstNode,
        )
    };
    assert!(!const_num4.is_null());
    assert_eq!(unsafe { &*const_num4 }.value, 1.5e-5);

    let expr5 = unsafe { &*(*return_stat.list.data.add(5)) };
    let const_num5 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprConstantNumber>(
            expr5 as *const AstExpr as *mut AstNode,
        )
    };
    assert!(!const_num5.is_null());
    assert_eq!(unsafe { &*const_num5 }.value, 12345.125);

    if FFlag::LuauIntegerType2.get() {
        let stat2 = fixture.parse("return 1i, 1_000_000i", &ParseOptions::default());
        assert!(!stat2.is_null());

        let block2 = unsafe { &*stat2 };
        assert!(!block2.body.data.is_null());

        let first_stat2 = unsafe { *block2.body.data.add(0) };
        let return_stat2 =
            unsafe { luaur_ast::rtti::ast_node_as::<AstStatReturn>(first_stat2 as *mut AstNode) };
        assert!(!return_stat2.is_null());

        let return_stat2 = unsafe { &*return_stat2 };
        assert_eq!(return_stat2.list.size, 2);

        let expr_int0 = unsafe { &*(*return_stat2.list.data.add(0)) };
        assert!(luaur_ast::rtti::ast_node_is::<AstExprConstantInteger>(
            expr_int0
        ));

        let const_int0 = unsafe {
            luaur_ast::rtti::ast_node_as::<AstExprConstantInteger>(
                expr_int0 as *const AstExpr as *mut AstNode,
            )
        };
        assert!(!const_int0.is_null());
        assert_eq!(unsafe { &*const_int0 }.value, 1);

        let expr_int1 = unsafe { &*(*return_stat2.list.data.add(1)) };
        assert!(luaur_ast::rtti::ast_node_is::<AstExprConstantInteger>(
            expr_int1
        ));

        let const_int1 = unsafe {
            luaur_ast::rtti::ast_node_as::<AstExprConstantInteger>(
                expr_int1 as *const AstExpr as *mut AstNode,
            )
        };
        assert!(!const_int1.is_null());
        assert_eq!(unsafe { &*const_int1 }.value, 1000000);
    }
}
