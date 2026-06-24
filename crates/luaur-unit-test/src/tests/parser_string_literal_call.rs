#[cfg(test)]
#[test]
fn parser_string_literal_call() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_expr_call::AstExprCall;
    use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_expr::AstStatExpr;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("do foo 'bar' end", &ParseOptions::default());
    let root = unsafe { &*stat };
    assert!(!root.body.data.is_null());

    let dob = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatBlock>(
            *root.body.data as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!dob.is_null());

    let stc = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatExpr>(
            *(*dob).body.data as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!stc.is_null());

    let ec = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprCall>(
            (*stc).expr as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!ec.is_null());

    assert_eq!(unsafe { (*ec).args.size }, 1);

    let arg = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprConstantString>(
            *(*ec).args.data as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!arg.is_null());

    let s = unsafe {
        let p = (*arg).value.data as *const u8;
        core::slice::from_raw_parts(p, (*arg).value.size as usize)
    };
    assert_eq!(core::str::from_utf8(s).unwrap(), "bar");
}
