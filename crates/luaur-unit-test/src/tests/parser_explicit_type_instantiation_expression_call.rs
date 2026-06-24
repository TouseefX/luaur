#[cfg(test)]
#[test]
fn parser_explicit_type_instantiation_expression_call() {
    use crate::functions::string_at_location::string_at_location;
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_expr::AstExpr;
    use luaur_ast::records::ast_expr_call::AstExprCall;
    use luaur_ast::records::ast_expr_instantiate::AstExprInstantiate;
    use luaur_ast::records::ast_stat_local::AstStatLocal;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("local x = f<<T, U>>()");
    let options = ParseOptions::parse_options();
    let result: ParseResult = fixture.parse_ex(&source, &options);

    assert!(!result.root.is_null());

    let local = unsafe { (*result.root).body.data.add(0) };
    let local = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatLocal>(
            *local as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!local.is_null());

    assert_eq!(1, unsafe { (*local).vars.size });

    let expr = unsafe { (*local).values.data.add(0) };
    assert!(!expr.is_null());

    let call = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprCall>(
            *expr as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!call.is_null());

    let explicit_type_instantiation = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprInstantiate>(
            (*call).func as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!explicit_type_instantiation.is_null());

    let location = unsafe { &(*explicit_type_instantiation).base.base.location };
    let expected = string_at_location(&source, location);
    assert_eq!("f<<T, U>>", expected);
}
