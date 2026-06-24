#[cfg(test)]
#[test]
fn parser_generic_pack_parsing() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let result = fixture.parse_ex(
        &alloc::string::String::from(
            "function f<a...>(...: a...)\n\
             end\n\
             \n\
             type A = (a...) -> b...",
        ),
        &ParseOptions::default(),
    );
    let root = result.root;
    assert!(!root.is_null());

    let stat_block = unsafe { &*root };
    assert!(!stat_block.body.data.is_null());
    assert!(stat_block.body.size > 0);

    let first_stat = unsafe { *stat_block.body.data.add(0) };
    let fn_stat = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_function::AstStatFunction>(
            first_stat as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!fn_stat.is_null());
    let fn_stat = unsafe { &*fn_stat };
    assert!(!fn_stat.func.is_null());
    let func = unsafe { &*fn_stat.func };
    assert!(!func.vararg_annotation.is_null());

    let vararg_annot = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_pack_generic::AstTypePackGeneric>(
            func.vararg_annotation as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!vararg_annot.is_null());
    let vararg_annot = unsafe { &*vararg_annot };
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr(vararg_annot.generic_name.value).to_string_lossy() },
        "a"
    );

    let second_stat = unsafe { *stat_block.body.data.add(1) };
    let alias_stat = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias>(
            second_stat as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!alias_stat.is_null());
    let alias_stat = unsafe { &*alias_stat };
    assert!(!alias_stat.type_ptr.is_null());
    let type_ptr = unsafe { &*alias_stat.type_ptr };
    let fn_ty = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_function::AstTypeFunction>(
            type_ptr as *const _ as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!fn_ty.is_null());
    let fn_ty = unsafe { &*fn_ty };

    let arg_type_pack = fn_ty.arg_types.tail_type;
    assert!(!arg_type_pack.is_null());
    let arg_annot = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_pack_generic::AstTypePackGeneric>(
            arg_type_pack as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!arg_annot.is_null());
    let arg_annot = unsafe { &*arg_annot };
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr(arg_annot.generic_name.value).to_string_lossy() },
        "a"
    );

    let ret_type_pack = fn_ty.return_types;
    assert!(!ret_type_pack.is_null());
    let ret_annot = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_pack_generic::AstTypePackGeneric>(
            ret_type_pack as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!ret_annot.is_null());
    let ret_annot = unsafe { &*ret_annot };
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr(ret_annot.generic_name.value).to_string_lossy() },
        "b"
    );
}
