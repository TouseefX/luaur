#[cfg(test)]
#[test]
fn parser_functions_can_have_a_function_type_annotation() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_array::AstArray;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_function::AstStatFunction;
    use luaur_ast::records::ast_type::AstType;
    use luaur_ast::records::ast_type_function::AstTypeFunction;
    use luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
        "function f(): (number) -> nil return nil end",
        &ParseOptions::default(),
    );
    let root = unsafe { &*stat };
    assert!(!root.body.data.is_null());

    let first_stat = unsafe { *root.body.data.add(0) };
    let stat_func = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatFunction>(
            first_stat as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!stat_func.is_null());

    let func = unsafe { (*stat_func).func };
    assert!(!func.is_null());

    let return_annotation = unsafe { (*func).return_annotation };
    assert!(!return_annotation.is_null());

    let type_pack = unsafe {
        luaur_ast::rtti::ast_node_as::<AstTypePackExplicit>(
            return_annotation as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!type_pack.is_null());

    let type_list = unsafe { &(*type_pack).type_list };
    assert!(unsafe { (*type_list).tail_type }.is_null());

    let ret_types = unsafe { &(*type_list).types };
    assert_eq!(ret_types.size, 1);

    let fun_ty = unsafe {
        luaur_ast::rtti::ast_node_as::<AstTypeFunction>(
            *ret_types.data.add(0) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!fun_ty.is_null());
}
