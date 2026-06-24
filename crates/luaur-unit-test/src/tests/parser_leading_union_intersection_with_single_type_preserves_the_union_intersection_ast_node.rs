#[cfg(test)]
#[test]
fn parser_leading_union_intersection_with_single_type_preserves_the_union_intersection_ast_node() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let source = alloc::string::String::from(
        "type Foo = | string\n\
         type Bar = & number",
    );
    let block = fixture.parse(&source, &ParseOptions::parse_options());

    assert_eq!(2, unsafe { (*block).body.size });

    let alias1 = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias>(
            *(*block).body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!alias1.is_null());

    let union_type = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_union::AstTypeUnion>(
            (*alias1).type_ptr as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!union_type.is_null());
    assert_eq!(1, unsafe { (*union_type).types.size });

    let alias2 = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias>(
            *(*block).body.data.add(1) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!alias2.is_null());

    let intersection_type = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_intersection::AstTypeIntersection>(
            (*alias2).type_ptr as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!intersection_type.is_null());
    assert_eq!(1, unsafe { (*intersection_type).types.size });
}
