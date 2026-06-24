#[cfg(test)]
#[test]
fn parser_grouped_function_type() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_array::AstArray;
    use luaur_ast::records::ast_stat_local::AstStatLocal;
    use luaur_ast::records::ast_type_function::AstTypeFunction;
    use luaur_ast::records::ast_type_group::AstTypeGroup;
    use luaur_ast::records::ast_type_optional::AstTypeOptional;
    use luaur_ast::records::ast_type_reference::AstTypeReference;
    use luaur_ast::records::ast_type_union::AstTypeUnion;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("type X<T> = T\nlocal x: X<(() -> ())?>");
    let parse_options = luaur_ast::records::parse_options::ParseOptions::parse_options();
    let root = fixture.parse(&source, &parse_options);

    assert!(!root.is_null());
    assert_eq!(2, unsafe { (*root).body.size });

    let assignment = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatLocal>(
            *(*root).body.data.add(1) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!assignment.is_null());

    assert_eq!(1, unsafe { (*assignment).vars.size });
    assert_eq!(0, unsafe { (*assignment).values.size });

    let binding = unsafe { (*assignment).vars.data.add(0).read() };
    assert_eq!("x", unsafe {
        core::ffi::CStr::from_ptr((*binding).name.value).to_string_lossy()
    });

    let generic_ty = unsafe {
        luaur_ast::rtti::ast_node_as::<AstTypeReference>(
            (*binding).annotation as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!generic_ty.is_null());

    assert_eq!(1, unsafe { (*generic_ty).parameters.size });

    let param_ty = unsafe { (*generic_ty).parameters.data.add(0).read() };
    assert!(!param_ty.r#type.is_null());

    let union_ty = unsafe {
        luaur_ast::rtti::ast_node_as::<AstTypeUnion>(
            param_ty.r#type as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!union_ty.is_null());

    assert_eq!(2, unsafe { (*union_ty).types.size });

    let group_ty = unsafe {
        luaur_ast::rtti::ast_node_as::<AstTypeGroup>(
            *(*union_ty).types.data.add(0) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!group_ty.is_null());

    let function_ty = unsafe {
        luaur_ast::rtti::ast_node_as::<AstTypeFunction>(
            (*group_ty).type_ as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!function_ty.is_null());

    let optional_ty = unsafe {
        luaur_ast::rtti::ast_node_as::<AstTypeOptional>(
            *(*union_ty).types.data.add(1) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!optional_ty.is_null());
}
