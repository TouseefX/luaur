#[cfg(test)]
#[test]
fn parser_parse_numbers_hexadecimal() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
        "return 0xab, 0xAB05, 0xff_ff, 0xffffffffffffffff",
        &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    let block = unsafe { &*stat };
    assert!(!block.body.data.is_null());
    assert!(block.body.size > 0);

    let first_stat = unsafe { &*block.body.data };
    let return_stat = unsafe {
        &*luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_return::AstStatReturn>(
            *first_stat as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert_eq!(return_stat.list.size, 4);

    let expr0 = unsafe { &*return_stat.list.data.add(0) };
    let const_num0 = unsafe {
        &*luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber,
        >(*expr0 as *mut luaur_ast::records::ast_node::AstNode)
    };
    assert_eq!(const_num0.value, 0xab as f64);

    let expr1 = unsafe { &*return_stat.list.data.add(1) };
    let const_num1 = unsafe {
        &*luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber,
        >(*expr1 as *mut luaur_ast::records::ast_node::AstNode)
    };
    assert_eq!(const_num1.value, 0xAB05 as f64);

    let expr2 = unsafe { &*return_stat.list.data.add(2) };
    let const_num2 = unsafe {
        &*luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber,
        >(*expr2 as *mut luaur_ast::records::ast_node::AstNode)
    };
    assert_eq!(const_num2.value, 0xFFFF as f64);

    let expr3 = unsafe { &*return_stat.list.data.add(3) };
    let const_num3 = unsafe {
        &*luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber,
        >(*expr3 as *mut luaur_ast::records::ast_node::AstNode)
    };
    // C++ `double(ULLONG_MAX)` — u64::MAX as f64 (1.844e19), NOT f64::MAX (1.797e308).
    assert_eq!(const_num3.value, u64::MAX as f64);

    if luaur_common::FFlag::LuauIntegerType2.get() {
        let mut fixture = Fixture::default();
        let stat = fixture.parse(
            "return 0xabi, 0XAB05i, 0xff_ffi, 0x7fffffffffffffffi, 0x8000000000000000i, 0xffffffffffffffffi",
            &ParseOptions::default(),
        );
        assert!(!stat.is_null());

        let block = unsafe { &*stat };
        assert!(!block.body.data.is_null());
        assert!(block.body.size > 0);

        let first_stat = unsafe { &*block.body.data };
        let return_stat = unsafe {
            &*luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_return::AstStatReturn>(
                *first_stat as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        assert_eq!(return_stat.list.size, 6);

        let expr0 = unsafe { &*return_stat.list.data.add(0) };
        let const_int0 = unsafe {
            &*luaur_ast::rtti::ast_node_as::<
                luaur_ast::records::ast_expr_constant_integer::AstExprConstantInteger,
            >(*expr0 as *mut luaur_ast::records::ast_node::AstNode)
        };
        assert_eq!(const_int0.value, 0xab);

        let expr1 = unsafe { &*return_stat.list.data.add(1) };
        let const_int1 = unsafe {
            &*luaur_ast::rtti::ast_node_as::<
                luaur_ast::records::ast_expr_constant_integer::AstExprConstantInteger,
            >(*expr1 as *mut luaur_ast::records::ast_node::AstNode)
        };
        assert_eq!(const_int1.value, 0xAB05);

        let expr2 = unsafe { &*return_stat.list.data.add(2) };
        let const_int2 = unsafe {
            &*luaur_ast::rtti::ast_node_as::<
                luaur_ast::records::ast_expr_constant_integer::AstExprConstantInteger,
            >(*expr2 as *mut luaur_ast::records::ast_node::AstNode)
        };
        assert_eq!(const_int2.value, 0xFFFF);

        let expr3 = unsafe { &*return_stat.list.data.add(3) };
        let const_int3 = unsafe {
            &*luaur_ast::rtti::ast_node_as::<
                luaur_ast::records::ast_expr_constant_integer::AstExprConstantInteger,
            >(*expr3 as *mut luaur_ast::records::ast_node::AstNode)
        };
        assert_eq!(const_int3.value, std::i64::MAX);

        let expr4 = unsafe { &*return_stat.list.data.add(4) };
        let const_int4 = unsafe {
            &*luaur_ast::rtti::ast_node_as::<
                luaur_ast::records::ast_expr_constant_integer::AstExprConstantInteger,
            >(*expr4 as *mut luaur_ast::records::ast_node::AstNode)
        };
        assert_eq!(const_int4.value, std::i64::MIN);

        let expr5 = unsafe { &*return_stat.list.data.add(5) };
        let const_int5 = unsafe {
            &*luaur_ast::rtti::ast_node_as::<
                luaur_ast::records::ast_expr_constant_integer::AstExprConstantInteger,
            >(*expr5 as *mut luaur_ast::records::ast_node::AstNode)
        };
        assert_eq!(const_int5.value, -1);
    }
}
