#[cfg(test)]
#[test]
fn parser_parse_extern_type_declarations_missing_with() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from(
        "\n        declare extern type Foo\n            prop: number\n            function method(self, foo: number): string\n        end\n\n        declare extern type Bar extends Foo\n            prop2: string\n        end\n    ",
    );
    let options = ParseOptions::parse_options();
    let result = fixture.try_parse(&source, &options);

    assert_eq!(result.errors.len(), 2);

    assert_eq!(
        "Expected `with` keyword before listing properties of the external type, but got prop instead",
        result.errors[0].get_message()
    );
    assert_eq!(
        "Expected `with` keyword before listing properties of the external type, but got prop2 instead",
        result.errors[1].get_message()
    );

    let stat = unsafe { &*result.root };
    assert_eq!(stat.body.size, 2);

    let declared_extern_type = unsafe {
        luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType,
        >(*stat.body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode)
    };
    assert!(!declared_extern_type.is_null());
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr((*declared_extern_type).name.value) },
        c"Foo"
    );
    assert!(unsafe { (*declared_extern_type).super_name }.is_none());

    assert_eq!(unsafe { (*declared_extern_type).props.size }, 2);

    let prop = unsafe { *unsafe { (*declared_extern_type).props.data }.add(0) };
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr(prop.name.value) },
        c"prop"
    );
    assert_eq!(
        prop.name_location,
        Location::new(Position::new(2, 12), Position::new(2, 16))
    );
    assert!(!unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_reference::AstTypeReference>(
            prop.ty as *mut luaur_ast::records::ast_node::AstNode,
        )
    }
    .is_null());
    assert_eq!(
        prop.location,
        Location::new(Position::new(2, 12), Position::new(2, 24))
    );

    let method = unsafe { *unsafe { (*declared_extern_type).props.data }.add(1) };
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr(method.name.value) },
        c"method"
    );
    assert_eq!(
        method.name_location,
        Location::new(Position::new(3, 21), Position::new(3, 27))
    );
    assert!(!unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_function::AstTypeFunction>(
            method.ty as *mut luaur_ast::records::ast_node::AstNode,
        )
    }
    .is_null());
    assert_eq!(
        method.location,
        Location::new(Position::new(3, 12), Position::new(3, 54))
    );
    assert!(method.is_method);

    let subclass = unsafe {
        luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType,
        >(*stat.body.data.add(1) as *mut luaur_ast::records::ast_node::AstNode)
    };
    assert!(!subclass.is_null());
    assert!(unsafe { (*subclass).super_name }.is_some());
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr((*subclass).super_name.unwrap().value) },
        c"Foo"
    );
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr((*subclass).name.value) },
        c"Bar"
    );

    assert_eq!(unsafe { (*subclass).props.size }, 1);
    let prop2 = unsafe { *unsafe { (*subclass).props.data }.add(0) };
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr(prop2.name.value) },
        c"prop2"
    );
    assert_eq!(
        prop2.name_location,
        Location::new(Position::new(7, 12), Position::new(7, 17))
    );
    assert!(!unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_reference::AstTypeReference>(
            prop2.ty as *mut luaur_ast::records::ast_node::AstNode,
        )
    }
    .is_null());
    assert_eq!(
        prop2.location,
        Location::new(Position::new(7, 12), Position::new(7, 25))
    );
}
