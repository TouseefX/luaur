#[cfg(test)]
#[test]
fn parser_parse_class_declarations() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_declared_extern_type_property::AstDeclaredExternTypeProperty;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;
    use luaur_ast::records::ast_type_function::AstTypeFunction;
    use luaur_ast::records::ast_type_reference::AstTypeReference;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_ast::rtti::ast_node_as;
    use luaur_ast::rtti::ast_node_is;

    let mut fixture = Fixture::default();
    let result = fixture.parse_ex(
        // Faithful to the C++ R-string: leading newline + 8-space (declare/end) and
        // 12-space (prop/function) indentation; the port used tabs + line-continuations.
        &alloc::string::String::from(
            "\n        declare class Foo\n            prop: number\n            function method(self, foo: number): string\n        end\n\n        declare class Bar extends Foo\n            prop2: string\n        end\n    ",
        ),
        &ParseOptions::default(),
    );
    let root = result.root;
    assert!(!root.is_null());

    let root_block: *mut AstStatBlock =
        unsafe { ast_node_as::<AstStatBlock>(root as *mut luaur_ast::records::ast_node::AstNode) };
    assert!(!root_block.is_null());

    let body = unsafe { &(*root_block).body };
    assert_eq!(2, body.size);

    let stat0 = unsafe { *body.data.add(0) };
    assert!(!stat0.is_null());
    assert!(unsafe {
        ast_node_is::<AstStatDeclareExternType>(stat0 as *mut luaur_ast::records::ast_node::AstNode)
    });

    let extern_type_foo: *mut AstStatDeclareExternType = unsafe {
        ast_node_as::<AstStatDeclareExternType>(stat0 as *mut luaur_ast::records::ast_node::AstNode)
    };
    assert!(!extern_type_foo.is_null());

    let foo_name = unsafe { (*extern_type_foo).name };
    assert_eq!("Foo", unsafe {
        core::ffi::CStr::from_ptr(foo_name.value).to_string_lossy()
    });

    let foo_super_name = unsafe { (*extern_type_foo).super_name };
    assert!(foo_super_name.is_none());

    let foo_props = unsafe { &(*extern_type_foo).props };
    assert_eq!(2, foo_props.size);

    let prop0 = unsafe { *foo_props.data.add(0) };
    let prop0_name = prop0.name;
    assert_eq!("prop", unsafe {
        core::ffi::CStr::from_ptr(prop0_name.value).to_string_lossy()
    });

    let expected_prop_loc = Location::new(
        luaur_ast::records::position::Position::new(2, 12),
        luaur_ast::records::position::Position::new(2, 16),
    );
    assert_eq!(expected_prop_loc, prop0.name_location);

    let expected_prop_full_loc = Location::new(
        luaur_ast::records::position::Position::new(2, 12),
        luaur_ast::records::position::Position::new(2, 24),
    );
    assert_eq!(expected_prop_full_loc, prop0.location);

    assert!(unsafe {
        ast_node_is::<AstTypeReference>(prop0.ty as *mut luaur_ast::records::ast_node::AstNode)
    });

    let method_prop = unsafe { *foo_props.data.add(1) };
    let method_name = method_prop.name;
    assert_eq!("method", unsafe {
        core::ffi::CStr::from_ptr(method_name.value).to_string_lossy()
    });

    let expected_method_name_loc = Location::new(
        luaur_ast::records::position::Position::new(3, 21),
        luaur_ast::records::position::Position::new(3, 27),
    );
    assert_eq!(expected_method_name_loc, method_prop.name_location);

    let expected_method_full_loc = Location::new(
        luaur_ast::records::position::Position::new(3, 12),
        luaur_ast::records::position::Position::new(3, 54),
    );
    assert_eq!(expected_method_full_loc, method_prop.location);

    assert!(method_prop.is_method);
    assert!(unsafe {
        ast_node_is::<AstTypeFunction>(method_prop.ty as *mut luaur_ast::records::ast_node::AstNode)
    });

    let stat1 = unsafe { *body.data.add(1) };
    assert!(!stat1.is_null());
    assert!(unsafe {
        ast_node_is::<AstStatDeclareExternType>(stat1 as *mut luaur_ast::records::ast_node::AstNode)
    });

    let extern_type_bar: *mut AstStatDeclareExternType = unsafe {
        ast_node_as::<AstStatDeclareExternType>(stat1 as *mut luaur_ast::records::ast_node::AstNode)
    };
    assert!(!extern_type_bar.is_null());

    let bar_name = unsafe { (*extern_type_bar).name };
    assert_eq!("Bar", unsafe {
        core::ffi::CStr::from_ptr(bar_name.value).to_string_lossy()
    });

    let bar_super_name = unsafe { (*extern_type_bar).super_name };
    assert!(bar_super_name.is_some());
    let super_name_ptr = bar_super_name.unwrap().value;
    assert_eq!("Foo", unsafe {
        core::ffi::CStr::from_ptr(super_name_ptr).to_string_lossy()
    });

    let bar_props = unsafe { &(*extern_type_bar).props };
    assert_eq!(1, bar_props.size);

    let prop1 = unsafe { *bar_props.data.add(0) };
    let prop1_name = prop1.name;
    assert_eq!("prop2", unsafe {
        core::ffi::CStr::from_ptr(prop1_name.value).to_string_lossy()
    });

    let expected_prop2_loc = Location::new(
        luaur_ast::records::position::Position::new(7, 12),
        luaur_ast::records::position::Position::new(7, 17),
    );
    assert_eq!(expected_prop2_loc, prop1.name_location);

    let expected_prop2_full_loc = Location::new(
        luaur_ast::records::position::Position::new(7, 12),
        luaur_ast::records::position::Position::new(7, 25),
    );
    assert_eq!(expected_prop2_full_loc, prop1.location);

    assert!(unsafe {
        ast_node_is::<AstTypeReference>(prop1.ty as *mut luaur_ast::records::ast_node::AstNode)
    });
}
