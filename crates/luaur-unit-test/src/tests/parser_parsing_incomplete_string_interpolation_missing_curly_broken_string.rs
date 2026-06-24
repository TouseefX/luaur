#[cfg(test)]
#[test]
fn parser_parsing_incomplete_string_interpolation_missing_curly_broken_string() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("print(`{e.x} {e.a\n");
    let result: ParseResult = fixture.try_parse(
        &source,
        &luaur_ast::records::parse_options::ParseOptions::parse_options(),
    );

    assert_eq!(2, result.errors.len());

    let first_error = &result.errors[0];
    assert_eq!(
        "Malformed interpolated string; did you forget to add a '}'?",
        first_error.get_message()
    );
    assert_eq!(
        Location::new(Position::new(0, 16), Position::new(0, 17)),
        *first_error.get_location()
    );

    let root = result.root;
    assert!(!root.is_null());

    let body = unsafe { &(*root).body };
    assert_eq!(1, body.size);

    let first_stmt = unsafe { *body.data.add(0) };
    let expr_stat = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_expr::AstStatExpr>(
            first_stmt as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!expr_stat.is_null());

    let call_expr = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_expr_call::AstExprCall>(
            (*expr_stat).expr as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!call_expr.is_null());

    let interp_string = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_expr_interp_string::AstExprInterpString>(
            *(*call_expr).args.data.add(0) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!interp_string.is_null());

    assert_eq!(2, unsafe { (*interp_string).expressions.size });
    assert_eq!(Position::new(0, 6), unsafe {
        (*interp_string).base.base.location.begin
    });
    assert_eq!(Position::new(0, 17), unsafe {
        (*interp_string).base.base.location.end
    });
}
