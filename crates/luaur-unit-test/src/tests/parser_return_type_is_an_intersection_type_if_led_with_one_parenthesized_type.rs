#[cfg(test)]
#[test]
fn parser_return_type_is_an_intersection_type_if_led_with_one_parenthesized_type() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_local::AstStatLocal;
    use luaur_ast::records::ast_type_function::AstTypeFunction;
    use luaur_ast::records::ast_type_group::AstTypeGroup;
    use luaur_ast::records::ast_type_intersection::AstTypeIntersection;
    use luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
        "local f: (string) -> (string) & (number) -> (number)",
        &ParseOptions::default(),
    );
    assert!(!block.is_null());

    let local = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatLocal>(
            (*block).body.data.add(0).read() as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!local.is_null());

    let annotation = unsafe { (*(*local).vars.data.add(0).read()).annotation };
    let annotation = unsafe {
        luaur_ast::rtti::ast_node_as::<AstTypeFunction>(
            annotation as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!annotation.is_null());

    let return_types = unsafe { (*annotation).return_types };
    let return_type_pack = unsafe {
        luaur_ast::rtti::ast_node_as::<AstTypePackExplicit>(
            return_types as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!return_type_pack.is_null());

    let types = unsafe { (*return_type_pack).type_list.types };
    let first_type = unsafe {
        luaur_ast::rtti::ast_node_as::<AstTypeIntersection>(
            *types.data as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!first_type.is_null());

    let inner_types = unsafe { (*first_type).types };
    let first_inner = unsafe {
        luaur_ast::rtti::ast_node_as::<AstTypeGroup>(
            *inner_types.data as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!first_inner.is_null());

    let second_inner = unsafe {
        luaur_ast::rtti::ast_node_as::<AstTypeFunction>(
            inner_types.data.add(1).read() as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!second_inner.is_null());
}
