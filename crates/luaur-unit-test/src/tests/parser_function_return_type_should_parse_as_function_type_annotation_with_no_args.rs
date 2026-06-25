#[cfg(test)]
#[test]
fn parser_function_return_type_should_parse_as_function_type_annotation_with_no_args() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
        "function f(): () -> nil return nil end",
        &ParseOptions::default(),
    );
    assert!(!block.is_null());

    let root = unsafe { &*block };
    assert!(root.body.size > 0);

    let stat_func = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_function::AstStatFunction>(
            *root.body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!stat_func.is_null());

    let func = unsafe { &*stat_func }.func;
    assert!(!func.is_null());

    let return_annotation = unsafe { &*func }.return_annotation;
    assert!(!return_annotation.is_null());

    let type_pack = unsafe {
        luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit,
        >(return_annotation as *mut luaur_ast::records::ast_node::AstNode)
    };
    assert!(!type_pack.is_null());

    let type_list = unsafe { &*type_pack }.type_list.clone();
    assert!(type_list.tail_type.is_null());

    let ret_types = type_list.types;
    assert_eq!(ret_types.size, 1);

    let fun_ty = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_function::AstTypeFunction>(
            *ret_types.data.add(0) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!fun_ty.is_null());

    let arg_types = unsafe { &*fun_ty }.arg_types.clone();
    assert_eq!(arg_types.types.size, 0);
    assert!(arg_types.tail_type.is_null());

    let return_types = unsafe { &*fun_ty }.return_types;
    assert!(!return_types.is_null());

    let fun_return_pack = unsafe {
        luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit,
        >(return_types as *mut luaur_ast::records::ast_node::AstNode)
    };
    assert!(!fun_return_pack.is_null());
    assert!(unsafe { &*fun_return_pack }.type_list.tail_type.is_null());

    let ty = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_reference::AstTypeReference>(
            *unsafe { &*fun_return_pack }.type_list.types.data.add(0)
                as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!ty.is_null());

    let name = unsafe { &*ty }.name;
    let name_str = unsafe { core::ffi::CStr::from_ptr(name.value).to_string_lossy() };
    assert_eq!(name_str, "nil");
}
