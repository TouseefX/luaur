#[cfg(test)]
#[test]
fn parser_number_literals() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_return::AstStatReturn;
    use luaur_ast::rtti::AstNodeClass;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
        "return\n\
         1,\n\
         1.5,\n\
         .5,\n\
         12_34_56,\n\
         0x1234,\n\
         0b010101",
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );
    assert!(!stat.is_null());

    // C++ `stat->body.data[0]->as<AstStatReturn>()` — the return is the first body
    // element, not the block's own base node.
    let ret = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatReturn>(*(&*stat).body.data as *mut AstNode)
    };
    assert!(!ret.is_null());

    let ret_ref = unsafe { &*ret };
    assert_eq!(ret_ref.list.size, 6);

    let num0 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprConstantNumber>(
            *ret_ref.list.data.add(0) as *mut AstNode
        )
    };
    assert!(!num0.is_null());
    assert_eq!(unsafe { &*num0 }.value, 1.0);

    let num1 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprConstantNumber>(
            *ret_ref.list.data.add(1) as *mut AstNode
        )
    };
    assert!(!num1.is_null());
    assert_eq!(unsafe { &*num1 }.value, 1.5);

    let num2 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprConstantNumber>(
            *ret_ref.list.data.add(2) as *mut AstNode
        )
    };
    assert!(!num2.is_null());
    assert_eq!(unsafe { &*num2 }.value, 0.5);

    let num3 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprConstantNumber>(
            *ret_ref.list.data.add(3) as *mut AstNode
        )
    };
    assert!(!num3.is_null());
    assert_eq!(unsafe { &*num3 }.value, 123456.0);

    let num4 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprConstantNumber>(
            *ret_ref.list.data.add(4) as *mut AstNode
        )
    };
    assert!(!num4.is_null());
    assert_eq!(unsafe { &*num4 }.value, 0x1234 as f64);

    let num5 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprConstantNumber>(
            *ret_ref.list.data.add(5) as *mut AstNode
        )
    };
    assert!(!num5.is_null());
    assert_eq!(unsafe { &*num5 }.value, 0x15 as f64);
}
