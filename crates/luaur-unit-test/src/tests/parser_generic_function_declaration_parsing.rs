#[cfg(test)]
#[test]
fn parser_generic_function_declaration_parsing() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let result = fixture.parse_ex(
        &alloc::string::String::from("declare function f<a, b, c...>()"),
        &ParseOptions::default(),
    );

    let root = unsafe { &*result.root };
    assert!(!root.body.data.is_null());

    let decl = unsafe {
        luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_stat_declare_function::AstStatDeclareFunction,
        >((*root.body.data) as *mut luaur_ast::records::ast_node::AstNode)
    };
    assert!(!decl.is_null());

    let decl_ref = unsafe { &*decl };
    assert_eq!(2, decl_ref.generics.size);
    assert_eq!(1, decl_ref.generic_packs.size);
}
