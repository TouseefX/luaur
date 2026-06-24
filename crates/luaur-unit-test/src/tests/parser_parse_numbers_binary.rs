#[cfg(test)]
#[test]
fn parser_parse_numbers_binary() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_array::AstArray;
    use luaur_ast::records::ast_expr::AstExpr;
    use luaur_ast::records::ast_expr_constant_integer::AstExprConstantInteger;
    use luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_return::AstStatReturn;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::rtti::AstNodeClass;
    use luaur_common::FFlag;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
        "return 0b1, 0b0, 0b101010, 0b1111111111111111111111111111111111111111111111111111111111111111",
        &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    let block = unsafe { &*stat };
    assert!(!block.body.data.is_null());
    assert!(block.body.size > 0);

    let first_stat = unsafe { &*block.body.data };
    let return_stat = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatReturn>(
            *first_stat as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!return_stat.is_null());
    let return_stat = unsafe { &*return_stat };

    assert_eq!(return_stat.list.size, 4);

    let expr0 = unsafe { &*return_stat.list.data.add(0) };
    let const_num0 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprConstantNumber>(
            *expr0 as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!const_num0.is_null());
    assert_eq!(unsafe { &*const_num0 }.value, 1.0);

    let expr1 = unsafe { &*return_stat.list.data.add(1) };
    let const_num1 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprConstantNumber>(
            *expr1 as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!const_num1.is_null());
    assert_eq!(unsafe { &*const_num1 }.value, 0.0);

    let expr2 = unsafe { &*return_stat.list.data.add(2) };
    let const_num2 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprConstantNumber>(
            *expr2 as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!const_num2.is_null());
    assert_eq!(unsafe { &*const_num2 }.value, 42.0);

    let expr3 = unsafe { &*return_stat.list.data.add(3) };
    let const_num3 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprConstantNumber>(
            *expr3 as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!const_num3.is_null());
    assert_eq!(unsafe { &*const_num3 }.value, u64::MAX as f64);

    if FFlag::LuauIntegerType2.get() {
        let mut fixture = Fixture::default();
        let stat = fixture.parse(
            "return 0b1i, 0b0i, 0b101010i, 0b111111111111111111111111111111111111111111111111111111111111111i, 0b1000000000000000000000000000000000000000000000000000000000000000i, 0b1111111111111111111111111111111111111111111111111111111111111111i",
            &ParseOptions::default(),
        );
        assert!(!stat.is_null());

        let block = unsafe { &*stat };
        assert!(!block.body.data.is_null());
        assert!(block.body.size > 0);

        let first_stat = unsafe { &*block.body.data };
        let return_stat = unsafe {
            luaur_ast::rtti::ast_node_as::<AstStatReturn>(
                *first_stat as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        assert!(!return_stat.is_null());
        let return_stat = unsafe { &*return_stat };

        assert_eq!(return_stat.list.size, 6);

        let expr0 = unsafe { &*return_stat.list.data.add(0) };
        let const_int0 = unsafe {
            luaur_ast::rtti::ast_node_as::<AstExprConstantInteger>(
                *expr0 as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        assert!(!const_int0.is_null());
        assert_eq!(unsafe { &*const_int0 }.value, 1);

        let expr1 = unsafe { &*return_stat.list.data.add(1) };
        let const_int1 = unsafe {
            luaur_ast::rtti::ast_node_as::<AstExprConstantInteger>(
                *expr1 as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        assert!(!const_int1.is_null());
        assert_eq!(unsafe { &*const_int1 }.value, 0);

        let expr2 = unsafe { &*return_stat.list.data.add(2) };
        let const_int2 = unsafe {
            luaur_ast::rtti::ast_node_as::<AstExprConstantInteger>(
                *expr2 as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        assert!(!const_int2.is_null());
        assert_eq!(unsafe { &*const_int2 }.value, 42);

        let expr3 = unsafe { &*return_stat.list.data.add(3) };
        let const_int3 = unsafe {
            luaur_ast::rtti::ast_node_as::<AstExprConstantInteger>(
                *expr3 as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        assert!(!const_int3.is_null());
        assert_eq!(unsafe { &*const_int3 }.value, i64::MAX);

        let expr4 = unsafe { &*return_stat.list.data.add(4) };
        let const_int4 = unsafe {
            luaur_ast::rtti::ast_node_as::<AstExprConstantInteger>(
                *expr4 as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        assert!(!const_int4.is_null());
        assert_eq!(unsafe { &*const_int4 }.value, i64::MIN);

        let expr5 = unsafe { &*return_stat.list.data.add(5) };
        let const_int5 = unsafe {
            luaur_ast::rtti::ast_node_as::<AstExprConstantInteger>(
                *expr5 as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        assert!(!const_int5.is_null());
        assert_eq!(unsafe { &*const_int5 }.value, -1);
    }
}
