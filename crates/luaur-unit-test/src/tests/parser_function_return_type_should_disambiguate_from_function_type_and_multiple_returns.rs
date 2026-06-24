#[cfg(test)]
#[test]
fn parser_function_return_type_should_disambiguate_from_function_type_and_multiple_returns() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
        "function f(): (number, string) return 1, \"foo\" end",
        &ParseOptions::default(),
    );
    assert!(!block.is_null());

    let root = unsafe { &*block };
    assert!(root.body.size > 0);

    let stat_func = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_function::AstStatFunction>(
            (*root.body.data) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!stat_func.is_null());

    let func_expr = unsafe { &*(*stat_func).func };
    assert!((*func_expr).return_annotation.is_null() == false);

    let type_pack = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit>(
            (*(*stat_func).func).return_annotation as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!type_pack.is_null());

    let type_list = unsafe { &(*type_pack).type_list };
    assert!((*type_list).tail_type.is_null());

    let ret_types = &(*type_list).types;
    assert!(ret_types.size == 2);

    let ty0 = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_reference::AstTypeReference>(
            (*ret_types.data) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!ty0.is_null());
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr((*ty0).name.value) },
        c"number"
    );

    let ty1 = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_reference::AstTypeReference>(
            (*ret_types.data.add(1)) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!ty1.is_null());
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr((*ty1).name.value) },
        c"string"
    );
}
