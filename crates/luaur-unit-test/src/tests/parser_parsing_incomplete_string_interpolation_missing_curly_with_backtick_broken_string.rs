#[cfg(test)]
#[test]
fn parser_parsing_incomplete_string_interpolation_missing_curly_with_backtick_broken_string() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("print(`{e.x} {e.a`\n");
    let options = ParseOptions::parse_options();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(2, result.errors.len());

    let expected_location_0 = Location::new(Position::new(0, 17), Position::new(0, 18));
    assert_eq!(expected_location_0, *result.errors[0].get_location());
    assert_eq!(
        "Malformed interpolated string; did you forget to add a '}'?",
        result.errors[0].get_message()
    );

    let first = unsafe { *result.root.as_ref().unwrap().body.data.add(0) };
    let expr = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_expr::AstStatExpr>(
            first as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!expr.is_null());

    let call = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_expr_call::AstExprCall>(
            (*expr).expr as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!call.is_null());

    let interp_string = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_expr_interp_string::AstExprInterpString>(
            *(*call).args.data.add(0) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!interp_string.is_null());

    assert_eq!(2, unsafe { (*interp_string).expressions.size });

    let expected_location = Location::new(Position::new(0, 6), Position::new(0, 18));
    assert_eq!(expected_location, unsafe {
        (*interp_string).base.base.location
    });
}
