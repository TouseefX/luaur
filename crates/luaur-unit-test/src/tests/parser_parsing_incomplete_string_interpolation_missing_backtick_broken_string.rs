#[cfg(test)]
#[test]
fn parser_parsing_incomplete_string_interpolation_missing_backtick_broken_string() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_array::AstArray;
    use luaur_ast::records::ast_expr_call::AstExprCall;
    use luaur_ast::records::ast_expr_interp_string::AstExprInterpString;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_expr::AstStatExpr;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("print(`{e.x} {e.a}\n");
    let options = ParseOptions::parse_options();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(2, result.errors.len());

    let first_error = &result.errors[0];
    assert_eq!(
        "Malformed interpolated string; did you forget to add a '`'?",
        first_error.get_message()
    );
    assert_eq!(
        Location::new(Position::new(0, 17), Position::new(0, 18)),
        *first_error.get_location()
    );

    assert!(!result.root.is_null());
    let root_block: *mut AstStatBlock = result.root;
    assert_eq!(1, unsafe { (*root_block).body.size });

    let first_stat_ptr = unsafe { (*root_block).body.data.add(0) };
    let first_stat = unsafe { *first_stat_ptr };
    let stat_expr = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatExpr>(
            first_stat as *const luaur_ast::records::ast_node::AstNode
                as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!stat_expr.is_null());

    let expr_call = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprCall>(
            (*stat_expr).expr as *const luaur_ast::records::ast_node::AstNode
                as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!expr_call.is_null());

    let interp_string = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprInterpString>(
            *(*expr_call).args.data.add(0) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!interp_string.is_null());

    assert_eq!(2, unsafe { (*interp_string).expressions.size });
    assert_eq!(
        Location::new(Position::new(0, 6), Position::new(0, 18)),
        unsafe { (*interp_string).base.base.location }
    );
}
