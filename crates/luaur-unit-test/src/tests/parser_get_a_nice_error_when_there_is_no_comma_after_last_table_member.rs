#[cfg(test)]
#[test]
fn parser_get_a_nice_error_when_there_is_no_comma_after_last_table_member() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_expr_table::AstExprTable;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from(
        "\n        local t = {\n            first = 1\n\n        local ok = true\n        local good = ok == true\n    ",
    );
    let options = ParseOptions::parse_options();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(result.errors.len(), 1);

    let error_location = result.errors[0].get_location();
    let expected_location = Location::new(Position::new(4, 8), Position::new(4, 13));
    assert_eq!(*error_location, expected_location);

    let error_message = result.errors[0].get_message();
    assert_eq!(
        *error_message,
        "Expected '}' (to close '{' at line 2), got 'local'"
    );

    assert_eq!(unsafe { (*result.root).body.size }, 3);

    // C++ `Luau::query<AstExprTable>(root)` recursively finds the first table; here
    // it's the value of `local t = { ... }` — body[0] (AstStatLocal).values[0].
    // (The AstQueryDsl `query`/findNthOccurenceOf visitor is still a todo! stub.)
    let table: *mut AstExprTable = unsafe {
        let local = luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_local::AstStatLocal>(
            *(*result.root).body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode,
        );
        luaur_ast::rtti::ast_node_as::<AstExprTable>(
            *(*local).values.data.add(0) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!table.is_null());

    assert_eq!(unsafe { (*table).items.size }, 1);
}
