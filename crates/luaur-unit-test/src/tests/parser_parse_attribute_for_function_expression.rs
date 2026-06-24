#[cfg(test)]
#[test]
fn parser_parse_attribute_for_function_expression() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_attr::AstAttr;
    use luaur_ast::records::ast_attr::AstAttrType;
    use luaur_ast::records::ast_expr_call::AstExprCall;
    use luaur_ast::records::ast_expr_function::AstExprFunction;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_expr::AstStatExpr;
    use luaur_ast::records::ast_stat_local::AstStatLocal;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let source1 = alloc::string::String::from(
        "\nlocal function invoker(f)\n    return f(1)\nend\n\ninvoker(@checked function(x) return (x + 2) end)\n",
    );
    let options = ParseOptions::parse_options();
    let stat1 = fixture.parse(&source1, &options);

    luaur_common::LUAU_ASSERT!(unsafe { !stat1.is_null() });

    let stat1_block = unsafe { &*stat1 };
    let body1 = stat1_block.body;
    let stmt2_ptr = unsafe { *body1.data.add(1) };
    let expr_stmt = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatExpr>(
            stmt2_ptr as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    luaur_common::LUAU_ASSERT!(unsafe { !expr_stmt.is_null() });
    let expr_stmt = unsafe { &*expr_stmt };
    let call_ptr = expr_stmt.expr;
    let call = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprCall>(
            call_ptr as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    luaur_common::LUAU_ASSERT!(unsafe { !call.is_null() });
    let call = unsafe { &*call };
    let args = call.args;
    let arg0_ptr = unsafe { *args.data.add(0) };
    let func_expr = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprFunction>(
            arg0_ptr as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    luaur_common::LUAU_ASSERT!(unsafe { !func_expr.is_null() });
    let func_expr = unsafe { &*func_expr };
    let attributes1 = func_expr.attributes;

    luaur_common::LUAU_ASSERT!(attributes1.size == 1);

    let attr0 = unsafe { &**attributes1.data.add(0) };
    let expected_location1 = Location::new(Position::new(5, 8), Position::new(5, 16));
    crate::functions::check_attribute::check_attribute(
        attr0,
        AstAttrType::Checked,
        expected_location1,
    );

    let source2 =
        alloc::string::String::from("\nlocal f = @checked function(x) return (x + 2) end\n");
    let stat2 = fixture.parse(&source2, &options);

    luaur_common::LUAU_ASSERT!(unsafe { !stat2.is_null() });

    let stat2_block = unsafe { &*stat2 };
    let body2 = stat2_block.body;
    let stmt0_ptr = unsafe { *body2.data.add(0) };
    let local_stmt = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatLocal>(
            stmt0_ptr as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    luaur_common::LUAU_ASSERT!(unsafe { !local_stmt.is_null() });
    let local_stmt = unsafe { &*local_stmt };
    let values = local_stmt.values;
    let val0_ptr = unsafe { *values.data.add(0) };
    let func_expr2 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprFunction>(
            val0_ptr as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    luaur_common::LUAU_ASSERT!(unsafe { !func_expr2.is_null() });
    let func_expr2 = unsafe { &*func_expr2 };
    let attributes2 = func_expr2.attributes;

    luaur_common::LUAU_ASSERT!(attributes2.size == 1);

    let attr0_2 = unsafe { &**attributes2.data.add(0) };
    let expected_location2 = Location::new(Position::new(1, 10), Position::new(1, 18));
    crate::functions::check_attribute::check_attribute(
        attr0_2,
        AstAttrType::Checked,
        expected_location2,
    );
}
