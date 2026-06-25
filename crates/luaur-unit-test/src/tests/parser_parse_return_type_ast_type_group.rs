#[cfg(test)]
#[test]
fn parser_parse_return_type_ast_type_group() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("type Foo = () -> (string)\n", &ParseOptions::default());
    let root = unsafe { &*stat };
    assert_eq!(1, root.body.size);

    let alias1 = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias>(
            *root.body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!alias1.is_null());

    let func_type = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_function::AstTypeFunction>(
            (*alias1).type_ptr as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!func_type.is_null());

    let return_type_pack = unsafe {
        luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit,
        >((*func_type).return_types as *mut luaur_ast::records::ast_node::AstNode)
    };
    assert!(!return_type_pack.is_null());
    assert_eq!(1, unsafe { (*return_type_pack).type_list.types.size });
    assert!(unsafe { (*return_type_pack).type_list.tail_type }.is_null());
    assert!(
        unsafe {
            luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_group::AstTypeGroup>(
                *(*return_type_pack).type_list.types.data.add(0)
                    as *mut luaur_ast::records::ast_node::AstNode,
            )
        }
        .is_null()
            == false
    );
}
