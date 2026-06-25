#[cfg(test)]
#[test]
fn parser_incomplete_method_call_still_yields_an_ast_expr_index_name() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_expr_error::AstExprError;
    use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
    use luaur_ast::records::ast_stat_error::AstStatError;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("game:\n");
    let options = ParseOptions::parse_options();
    let result: luaur_ast::records::parse_result::ParseResult =
        fixture.try_parse(&source, &options);

    assert_eq!(1, unsafe { (*result.root).body.size });

    let stat: *mut AstStatError = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatError>(
            *(*result.root).body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!stat.is_null());

    let expr: *mut AstExprError = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprError>(
            *(*stat).expressions.data.add(0) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!expr.is_null());

    let index_name: *mut AstExprIndexName = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprIndexName>(
            *(*expr).expressions.data.add(0) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!index_name.is_null());
}
