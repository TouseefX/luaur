#[cfg(test)]
#[test]
fn parser_inner_and_outer_scope_of_functions_have_correct_end_position() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from(
        "\n        local function foo()\n            local x = 1\n        end\n    ",
    );
    let options = ParseOptions::parse_options();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert!(!result.root.is_null());
    let stat_block: *mut AstStatBlock = result.root;
    assert_eq!(1, unsafe { (*stat_block).body.size });

    let first_stat = unsafe { *(*stat_block).body.data.add(0) };
    let func_stat: *mut AstStatLocalFunction = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatLocalFunction>(
            first_stat as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!func_stat.is_null());

    let func_expr = unsafe { (*func_stat).func };
    let body_location = unsafe { (*(*func_expr).body).base.base.location };
    let expected_body_location = Location::new(Position::new(1, 28), Position::new(3, 8));
    assert_eq!(expected_body_location, body_location);

    let stat_location = unsafe { (*func_stat).base.base.location };
    let expected_stat_location = Location::new(Position::new(1, 8), Position::new(3, 11));
    assert_eq!(expected_stat_location, stat_location);
}
