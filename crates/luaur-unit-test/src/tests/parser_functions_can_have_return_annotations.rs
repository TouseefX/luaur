#[cfg(test)]
#[test]
fn parser_functions_can_have_return_annotations() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_function::AstStatFunction;
    use luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
        "function foo(): number return 55 end",
        &ParseOptions::default(),
    );
    assert!(!block.is_null());

    let root = unsafe { &*block };
    assert!(root.body.size > 0);

    let stat_function = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatFunction>(
            (*root.body.data) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!stat_function.is_null());

    let func = unsafe { &*stat_function }.func;
    assert!(!func.is_null());

    let return_annotation = unsafe { &*func }.return_annotation;
    assert!(!return_annotation.is_null());

    let type_pack_explicit = unsafe {
        luaur_ast::rtti::ast_node_as::<AstTypePackExplicit>(
            return_annotation as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!type_pack_explicit.is_null());

    let type_list = unsafe { &*type_pack_explicit }.type_list;
    assert_eq!(type_list.types.size, 1);
    assert!(type_list.tail_type.is_null());
}
