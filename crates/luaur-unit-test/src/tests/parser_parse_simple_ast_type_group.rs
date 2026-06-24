#[cfg(test)]
#[test]
fn parser_parse_simple_ast_type_group() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("type Foo = (string)\n", &ParseOptions::default());
    let root = unsafe { &*stat };
    assert_eq!(1, root.body.size);

    let alias1 = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias>(
            *root.body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!alias1.is_null());

    let group1 = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_group::AstTypeGroup>(
            (*alias1).type_ptr as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!group1.is_null());

    let _ = unsafe {
        !luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_reference::AstTypeReference>(
            (*group1).type_ as *mut luaur_ast::records::ast_node::AstNode,
        )
        .is_null()
    };
}
