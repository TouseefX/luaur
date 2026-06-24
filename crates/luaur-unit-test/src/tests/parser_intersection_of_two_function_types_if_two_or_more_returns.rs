#[cfg(test)]
#[test]
fn parser_intersection_of_two_function_types_if_two_or_more_returns() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_local::AstStatLocal;
    use luaur_ast::records::ast_type_function::AstTypeFunction;
    use luaur_ast::records::ast_type_intersection::AstTypeIntersection;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
        "local f: (string) -> (string, number) & (number) -> (number, string)",
        &ParseOptions::default(),
    );
    assert!(!block.is_null());

    let stat_block = unsafe { &*block };
    assert!(!stat_block.body.data.is_null());
    let first_stat = unsafe { *stat_block.body.data.add(0) };
    assert!(!first_stat.is_null());

    let local = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatLocal>(
            first_stat as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!local.is_null());
    let local_ref = unsafe { &*local };

    assert!(!local_ref.vars.data.is_null());
    let first_var = unsafe { *local_ref.vars.data.add(0) };
    assert!(!first_var.is_null());
    let var_ref = unsafe { &*first_var };

    let annotation = var_ref.annotation;
    assert!(!annotation.is_null());

    let intersection = unsafe {
        luaur_ast::rtti::ast_node_as::<AstTypeIntersection>(
            annotation as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!intersection.is_null());
    let intersection_ref = unsafe { &*intersection };

    assert!(!intersection_ref.types.data.is_null());
    assert_eq!(intersection_ref.types.size, 2);

    let first_type = unsafe { *intersection_ref.types.data.add(0) };
    assert!(!first_type.is_null());
    let first_type_fn = unsafe {
        luaur_ast::rtti::ast_node_as::<AstTypeFunction>(
            first_type as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!first_type_fn.is_null());

    let second_type = unsafe { *intersection_ref.types.data.add(1) };
    assert!(!second_type.is_null());
    let second_type_fn = unsafe {
        luaur_ast::rtti::ast_node_as::<AstTypeFunction>(
            second_type as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!second_type_fn.is_null());
}
