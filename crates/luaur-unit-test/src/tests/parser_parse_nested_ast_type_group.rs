#[cfg(test)]
#[test]
fn parser_parse_nested_ast_type_group() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("type Foo = ((string))", &ParseOptions::default());
    let root = unsafe { &*stat };
    assert_eq!(1, root.body.size);

    let alias1 = unsafe { (*stat).body.data.add(0) };
    let alias1 = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias>(
            *alias1 as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!alias1.is_null());

    let group1 = unsafe { (*alias1).type_ptr };
    let group1 = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_group::AstTypeGroup>(
            group1 as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!group1.is_null());

    let group2 = unsafe { (*group1).type_ };
    let group2 = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_group::AstTypeGroup>(
            group2 as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!group2.is_null());

    let ref_node = unsafe { (*group2).type_ };
    assert!(unsafe {
        luaur_ast::rtti::ast_node_is::<luaur_ast::records::ast_type_reference::AstTypeReference>(
            ref_node as *mut luaur_ast::records::ast_node::AstNode,
        )
    });
}
