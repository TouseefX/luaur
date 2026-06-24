#[cfg(test)]
#[test]
fn parser_function_start_locations_are_before_attributes() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_expr_function::AstExprFunction;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_function::AstStatFunction;
    use luaur_ast::records::ast_stat_local::AstStatLocal;
    use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
        "\n        @native\n        function globalFunction()\n        end\n\n        @native\n        local function localFunction()\n        end\n\n        local _ = @native function()\n        end\n    ",
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );
    let root = unsafe { &*stat };
    assert_eq!(3, root.body.size);

    let global_function = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatFunction>(
            *root.body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!global_function.is_null());
    assert_eq!(
        Location::new(Position::new(1, 8), Position::new(3, 11)),
        unsafe { (*global_function).base.base.location }
    );

    let local_function = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatLocalFunction>(
            *root.body.data.add(1) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!local_function.is_null());
    assert_eq!(
        Location::new(Position::new(5, 8), Position::new(7, 11)),
        unsafe { (*local_function).base.base.location }
    );

    let local_variable = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatLocal>(
            *root.body.data.add(2) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!local_variable.is_null());
    assert_eq!(1, unsafe { (*local_variable).values.size });

    let anonymous_function = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprFunction>(
            *(*local_variable).values.data.add(0) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!anonymous_function.is_null());
    assert_eq!(
        Location::new(Position::new(9, 18), Position::new(10, 11)),
        unsafe { (*anonymous_function).base.base.location }
    );
}
