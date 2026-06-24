#[cfg(test)]
#[test]
fn parser_parse_if_else_expression() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_expr_if_else::AstExprIfElse;
    use luaur_ast::records::ast_stat::AstStat;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_return::AstStatReturn;
    use luaur_ast::records::parse_options::ParseOptions;

    {
        let mut fixture = Fixture::default();
        let stat = fixture.parse("return if true then 1 else 2", &ParseOptions::default());
        assert!(!stat.is_null());
        let block = unsafe {
            luaur_ast::rtti::ast_node_as::<AstStatBlock>(
                core::ptr::addr_of_mut!((*stat).base) as *mut luaur_ast::records::ast_node::AstNode
            )
        };
        assert!(!block.is_null());
        assert!((unsafe { (*block).body.size }) > 0);
        let return_stat = unsafe { (*block).body.data.add(0).read() };
        let return_stat = unsafe {
            luaur_ast::rtti::ast_node_as::<AstStatReturn>(core::ptr::addr_of_mut!(
                (*return_stat).base
            )
                as *mut luaur_ast::records::ast_node::AstNode)
        };
        assert!(!return_stat.is_null());
        assert!((unsafe { (*return_stat).list.size }) == 1);
        let expr = unsafe { (*return_stat).list.data.add(0).read() };
        let if_else_expr = unsafe {
            luaur_ast::rtti::ast_node_as::<AstExprIfElse>(
                core::ptr::addr_of_mut!((*expr).base) as *mut luaur_ast::records::ast_node::AstNode
            )
        };
        assert!(!if_else_expr.is_null());
    }

    {
        let mut fixture = Fixture::default();
        let stat = fixture.parse(
            "return if true then 1 elseif true then 2 else 3",
            &ParseOptions::default(),
        );
        assert!(!stat.is_null());
        let block = unsafe {
            luaur_ast::rtti::ast_node_as::<AstStatBlock>(
                core::ptr::addr_of_mut!((*stat).base) as *mut luaur_ast::records::ast_node::AstNode
            )
        };
        assert!(!block.is_null());
        assert!((unsafe { (*block).body.size }) > 0);
        let return_stat = unsafe { (*block).body.data.add(0).read() };
        let return_stat = unsafe {
            luaur_ast::rtti::ast_node_as::<AstStatReturn>(core::ptr::addr_of_mut!(
                (*return_stat).base
            )
                as *mut luaur_ast::records::ast_node::AstNode)
        };
        assert!(!return_stat.is_null());
        assert!((unsafe { (*return_stat).list.size }) == 1);
        let expr = unsafe { (*return_stat).list.data.add(0).read() };
        let if_else_expr1 = unsafe {
            luaur_ast::rtti::ast_node_as::<AstExprIfElse>(
                core::ptr::addr_of_mut!((*expr).base) as *mut luaur_ast::records::ast_node::AstNode
            )
        };
        assert!(!if_else_expr1.is_null());
        let false_expr = unsafe { (*if_else_expr1).false_expr };
        let if_else_expr2 = unsafe {
            luaur_ast::rtti::ast_node_as::<AstExprIfElse>(
                core::ptr::addr_of_mut!((*false_expr).base)
                    as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        assert!(!if_else_expr2.is_null());
    }

    {
        let mut fixture = Fixture::default();
        let stat = fixture.parse(
            "return if true then 1 else if true then 2 else 3",
            &ParseOptions::default(),
        );
        assert!(!stat.is_null());
        let block = unsafe {
            luaur_ast::rtti::ast_node_as::<AstStatBlock>(
                core::ptr::addr_of_mut!((*stat).base) as *mut luaur_ast::records::ast_node::AstNode
            )
        };
        assert!(!block.is_null());
        assert!((unsafe { (*block).body.size }) > 0);
        let return_stat = unsafe { (*block).body.data.add(0).read() };
        let return_stat = unsafe {
            luaur_ast::rtti::ast_node_as::<AstStatReturn>(core::ptr::addr_of_mut!(
                (*return_stat).base
            )
                as *mut luaur_ast::records::ast_node::AstNode)
        };
        assert!(!return_stat.is_null());
        assert!((unsafe { (*return_stat).list.size }) == 1);
        let expr = unsafe { (*return_stat).list.data.add(0).read() };
        let if_else_expr1 = unsafe {
            luaur_ast::rtti::ast_node_as::<AstExprIfElse>(
                core::ptr::addr_of_mut!((*expr).base) as *mut luaur_ast::records::ast_node::AstNode
            )
        };
        assert!(!if_else_expr1.is_null());
        let false_expr = unsafe { (*if_else_expr1).false_expr };
        let if_else_expr2 = unsafe {
            luaur_ast::rtti::ast_node_as::<AstExprIfElse>(
                core::ptr::addr_of_mut!((*false_expr).base)
                    as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        assert!(!if_else_expr2.is_null());
    }

    {
        let mut fixture = Fixture::default();
        let stat = fixture.parse(
            "return if if true then false else true then 1 else 2",
            &ParseOptions::default(),
        );
        assert!(!stat.is_null());
        let block = unsafe {
            luaur_ast::rtti::ast_node_as::<AstStatBlock>(
                core::ptr::addr_of_mut!((*stat).base) as *mut luaur_ast::records::ast_node::AstNode
            )
        };
        assert!(!block.is_null());
        assert!((unsafe { (*block).body.size }) > 0);
        let return_stat = unsafe { (*block).body.data.add(0).read() };
        let return_stat = unsafe {
            luaur_ast::rtti::ast_node_as::<AstStatReturn>(core::ptr::addr_of_mut!(
                (*return_stat).base
            )
                as *mut luaur_ast::records::ast_node::AstNode)
        };
        assert!(!return_stat.is_null());
        assert!((unsafe { (*return_stat).list.size }) == 1);
        let expr = unsafe { (*return_stat).list.data.add(0).read() };
        let if_else_expr = unsafe {
            luaur_ast::rtti::ast_node_as::<AstExprIfElse>(
                core::ptr::addr_of_mut!((*expr).base) as *mut luaur_ast::records::ast_node::AstNode
            )
        };
        assert!(!if_else_expr.is_null());
        let condition = unsafe { (*if_else_expr).condition };
        let nested_if_else_expr = unsafe {
            luaur_ast::rtti::ast_node_as::<AstExprIfElse>(core::ptr::addr_of_mut!((*condition).base)
                as *mut luaur_ast::records::ast_node::AstNode)
        };
        assert!(!nested_if_else_expr.is_null());
    }
}
