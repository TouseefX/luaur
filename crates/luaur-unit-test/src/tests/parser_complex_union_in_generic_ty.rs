#[cfg(test)]
#[test]
fn parser_complex_union_in_generic_ty() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from(
        "type X<T> = T\n\
         local x: X<\n\
             | number\n\
             | boolean\n\
             | string\n\
         >",
    );

    let result = fix.try_parse(&code, &ParseOptions::parse_options());

    assert_eq!(result.errors.len(), 0);

    let block = unsafe { &*result.root };
    assert_eq!(block.body.size, 2);

    let stat_1 = unsafe { *block.body.data.add(1) };
    let assignment = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_local::AstStatLocal>(
            stat_1 as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!assignment.is_null());

    let assignment = unsafe { &*assignment };
    assert_eq!(assignment.vars.size, 1);
    assert_eq!(assignment.values.size, 0);

    let var_0 = unsafe { &**assignment.vars.data.add(0) };
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr((*var_0).name.value) },
        c"x"
    );

    let annotation = unsafe { &*(*var_0).annotation };
    let generic_ty = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_reference::AstTypeReference>(
            annotation as *const _ as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!generic_ty.is_null());

    let generic_ty = unsafe { &*generic_ty };
    assert_eq!(generic_ty.parameters.size, 1);

    let param_0 = unsafe { *generic_ty.parameters.data.add(0) };
    let param_ty = unsafe { &*param_0.r#type };
    let union_ty = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_union::AstTypeUnion>(
            param_ty as *const _ as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!union_ty.is_null());

    let union_ty = unsafe { &*union_ty };
    assert_eq!(union_ty.types.size, 3);

    let expected_types: [&core::ffi::CStr; 3] = [c"number", c"boolean", c"string"];
    for i in 0..3 {
        let ty = unsafe { &*(*union_ty.types.data.add(i)) };
        let ty_ref = unsafe {
            luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_reference::AstTypeReference>(
                ty as *const _ as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        assert!(!ty_ref.is_null());
        assert_eq!(
            unsafe { core::ffi::CStr::from_ptr((*ty_ref).name.value) },
            expected_types[i as usize]
        );
    }
}
