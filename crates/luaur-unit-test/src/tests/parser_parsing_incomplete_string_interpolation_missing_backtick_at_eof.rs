#[cfg(test)]
#[test]
fn parser_parsing_incomplete_string_interpolation_missing_backtick_at_eof() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("print(`{e.x} {e.a}");
    let options = ParseOptions::parse_options();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(2, result.errors.len());

    let first_error = &result.errors[0];
    assert_eq!(
        "Malformed interpolated string; did you forget to add a '`'?",
        first_error.get_message()
    );
    let expected_location = Location::new(Position::new(0, 17), Position::new(0, 18));
    assert_eq!(expected_location, *first_error.get_location());

    assert!(!result.root.is_null());
    let body = unsafe { (*result.root).body };
    assert_eq!(1, body.size);

    let first_stmt = unsafe { *body.data.add(0) };
    let stat_expr = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_expr::AstStatExpr>(
            first_stmt as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!stat_expr.is_null());

    let call = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_expr_call::AstExprCall>(
            (*stat_expr).expr as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!call.is_null());

    let args = unsafe { (*call).args };
    assert_eq!(1, args.size);

    let interp_string = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_expr_interp_string::AstExprInterpString>(
            *args.data.add(0) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!interp_string.is_null());

    let expressions = unsafe { (*interp_string).expressions };
    assert_eq!(2, expressions.size);

    let interp_string_location = unsafe { (*interp_string).base.base.location };
    let expected_begin = Position::new(0, 6);
    let expected_end = Position::new(0, 18);
    assert_eq!(expected_begin, interp_string_location.begin);
    assert_eq!(expected_end, interp_string_location.end);
}
