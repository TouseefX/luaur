#[cfg(test)]
#[test]
fn parser_parse_extern_type_declarations() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_declared_extern_type_property::AstDeclaredExternTypeProperty;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;
    use luaur_ast::records::ast_type_function::AstTypeFunction;
    use luaur_ast::records::ast_type_reference::AstTypeReference;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_ast::rtti::AstNodeClass;

    let mut fixture = Fixture::default();
    // Faithful to the C++ R"(...)" — leading newline + 8-space (declare/end) and
    // 12-space (prop/function) indentation, which the asserted Locations depend on.
    let source = alloc::string::String::from(
        "\n        declare extern type Foo with\n            prop: number\n            function method(self, foo: number): string\n        end\n\n        declare extern type Bar extends Foo with\n            prop2: string\n        end\n    ",
    );
    let result: ParseResult = fixture.parse_ex(&source, &ParseOptions::default());
    let root: *mut AstStatBlock = result.root;

    assert!(!root.is_null());
    let root_ref = unsafe { &*root };
    assert_eq!(2, root_ref.body.size);

    let first_stat: *mut luaur_ast::records::ast_stat::AstStat =
        unsafe { *root_ref.body.data.add(0) };
    let first_stat_node: *mut AstNode = first_stat as *mut AstNode;
    let declared_extern_type: *mut AstStatDeclareExternType =
        unsafe { AstNode::as_item_mut::<AstStatDeclareExternType>(&mut *first_stat_node) };
    assert!(!declared_extern_type.is_null());

    let det = unsafe { &*declared_extern_type };
    assert_eq!("Foo", unsafe {
        core::ffi::CStr::from_ptr(det.name.value).to_string_lossy()
    });
    assert!(det.super_name.is_none());

    assert_eq!(2, det.props.size);

    let prop = unsafe { *det.props.data.add(0) };
    assert_eq!("prop", unsafe {
        core::ffi::CStr::from_ptr(prop.name.value).to_string_lossy()
    });
    assert_eq!(
        Location::new(
            luaur_ast::records::position::Position::new(2, 12),
            luaur_ast::records::position::Position::new(2, 16)
        ),
        prop.name_location
    );
    assert!(unsafe {
        !luaur_ast::rtti::ast_node_as::<AstTypeReference>(
            prop.ty as *mut luaur_ast::records::ast_node::AstNode,
        )
        .is_null()
    });
    assert_eq!(
        Location::new(
            luaur_ast::records::position::Position::new(2, 12),
            luaur_ast::records::position::Position::new(2, 24)
        ),
        prop.location
    );

    let method = unsafe { *det.props.data.add(1) };
    assert_eq!("method", unsafe {
        core::ffi::CStr::from_ptr(method.name.value).to_string_lossy()
    });
    assert_eq!(
        Location::new(
            luaur_ast::records::position::Position::new(3, 21),
            luaur_ast::records::position::Position::new(3, 27)
        ),
        method.name_location
    );
    assert!(unsafe {
        !luaur_ast::rtti::ast_node_as::<AstTypeFunction>(
            method.ty as *mut luaur_ast::records::ast_node::AstNode,
        )
        .is_null()
    });
    assert_eq!(
        Location::new(
            luaur_ast::records::position::Position::new(3, 12),
            luaur_ast::records::position::Position::new(3, 54)
        ),
        method.location
    );
    assert!(method.is_method);

    let second_stat: *mut luaur_ast::records::ast_stat::AstStat =
        unsafe { *root_ref.body.data.add(1) };
    let second_stat_node: *mut AstNode = second_stat as *mut AstNode;
    let subclass: *mut AstStatDeclareExternType =
        unsafe { AstNode::as_item_mut::<AstStatDeclareExternType>(&mut *second_stat_node) };
    assert!(!subclass.is_null());

    let sub = unsafe { &*subclass };
    assert_eq!("Bar", unsafe {
        core::ffi::CStr::from_ptr(sub.name.value).to_string_lossy()
    });
    assert!(sub.super_name.is_some());
    assert_eq!("Foo", unsafe {
        core::ffi::CStr::from_ptr(sub.super_name.unwrap().value).to_string_lossy()
    });

    assert_eq!(1, sub.props.size);

    let prop2 = unsafe { *sub.props.data.add(0) };
    assert_eq!("prop2", unsafe {
        core::ffi::CStr::from_ptr(prop2.name.value).to_string_lossy()
    });
    assert_eq!(
        Location::new(
            luaur_ast::records::position::Position::new(7, 12),
            luaur_ast::records::position::Position::new(7, 17)
        ),
        prop2.name_location
    );
    assert!(unsafe {
        !luaur_ast::rtti::ast_node_as::<AstTypeReference>(
            prop2.ty as *mut luaur_ast::records::ast_node::AstNode,
        )
        .is_null()
    });
    assert_eq!(
        Location::new(
            luaur_ast::records::position::Position::new(7, 12),
            luaur_ast::records::position::Position::new(7, 25)
        ),
        prop2.location
    );
}
