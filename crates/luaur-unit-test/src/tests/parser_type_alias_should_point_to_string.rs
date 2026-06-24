#[cfg(test)]
#[test]
fn parser_type_alias_should_point_to_string() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let block = fixture.parse("type A = string", &ParseOptions::default());
    assert!(!block.is_null());
    let root = unsafe { &*block };
    assert!(root.body.size > 0);
    let first_stat = unsafe { &*root.body.data.add(0) };
    assert!(luaur_ast::rtti::ast_node_is::<
        luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias,
    >(
        *first_stat as *mut luaur_ast::records::ast_node::AstNode
    ));
}
