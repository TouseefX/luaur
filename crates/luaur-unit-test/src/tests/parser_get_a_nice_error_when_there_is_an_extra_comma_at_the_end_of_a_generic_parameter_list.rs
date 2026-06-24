#[cfg(test)]
#[test]
fn parser_get_a_nice_error_when_there_is_an_extra_comma_at_the_end_of_a_generic_parameter_list() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias;
    use luaur_ast::records::ast_type_function::AstTypeFunction;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from(
        "\n        export type VisitFn = <A, B,>(a: A, b: B) -> ()\n    ",
    );
    let options = ParseOptions::parse_options();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(result.errors.len(), 1);

    let error = &result.errors[0];
    let expected_location = Location::new(Position::new(1, 36), Position::new(1, 37));
    assert_eq!(*error.get_location(), expected_location);
    assert_eq!(
        error.get_message().as_str(),
        "Expected type after ',' but got '>' instead"
    );

    assert_eq!(unsafe { (*result.root).body.size }, 1);

    let stat = unsafe { *(*result.root).body.data.add(0) };
    let t: *mut AstStatTypeAlias = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatTypeAlias>(
            stat as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!t.is_null());

    // C++ navigates the alias's type (`->type`) — the function type — not the
    // alias node's own base.
    let f: *mut AstTypeFunction = unsafe {
        luaur_ast::rtti::ast_node_as::<AstTypeFunction>(
            (*t).type_ptr as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!f.is_null());

    assert_eq!(unsafe { (*f).generics.size }, 2);
}
