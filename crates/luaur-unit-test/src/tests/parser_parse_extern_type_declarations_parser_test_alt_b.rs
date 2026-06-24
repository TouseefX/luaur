//! Node: `cxx:Test:Luau.UnitTest:tests/Parser.test.cpp:2249:parse_extern_type_declarations`

#[cfg(test)]
#[test]
fn parser_parse_extern_type_declarations() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let result = fixture.parse_ex(
        &alloc::string::String::from(
            "\n        declare extern type Foo with\n            prop: number\n            function method(self, foo: number): string\n        end\n\n        declare extern type Bar extends Foo with\n            prop2: string\n        end\n    ",
        ),
        &ParseOptions::default(),
    );
    let root = unsafe { &*result.root };
    assert_eq!(2, root.body.size);

    let stat0 = unsafe { &*(*root.body.data.add(0)) };
    let declared_extern_type = unsafe {
        luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType,
        >(
            stat0 as *const luaur_ast::records::ast_stat::AstStat
                as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!declared_extern_type.is_null());
    assert_eq!(
        "Foo",
        unsafe {
            alloc::string::String::from_utf8_lossy(core::slice::from_raw_parts(
                (*declared_extern_type).name.value as *const u8,
                core::ffi::CStr::from_ptr((*declared_extern_type).name.value)
                    .to_bytes()
                    .len(),
            ))
        }
        .as_ref()
    );
    assert!(unsafe { (*declared_extern_type).super_name.is_none() });

    assert_eq!(2, unsafe { (*declared_extern_type).props.size });

    let prop = unsafe { *(*declared_extern_type).props.data.add(0) };
    assert_eq!(
        "prop",
        unsafe {
            alloc::string::String::from_utf8_lossy(core::slice::from_raw_parts(
                prop.name.value as *const u8,
                core::ffi::CStr::from_ptr(prop.name.value).to_bytes().len(),
            ))
        }
        .as_ref()
    );
    assert_eq!(
        luaur_ast::records::location::Location::new(
            luaur_ast::records::position::Position::new(2, 12),
            luaur_ast::records::position::Position::new(2, 16)
        ),
        prop.name_location
    );
    assert!(unsafe {
        luaur_ast::rtti::ast_node_is::<luaur_ast::records::ast_type_reference::AstTypeReference>(
            prop.ty as *mut luaur_ast::records::ast_node::AstNode,
        )
    });
    assert_eq!(
        luaur_ast::records::location::Location::new(
            luaur_ast::records::position::Position::new(2, 12),
            luaur_ast::records::position::Position::new(2, 24)
        ),
        prop.location
    );

    let method = unsafe { *(*declared_extern_type).props.data.add(1) };
    assert_eq!(
        "method",
        unsafe {
            alloc::string::String::from_utf8_lossy(core::slice::from_raw_parts(
                method.name.value as *const u8,
                core::ffi::CStr::from_ptr(method.name.value)
                    .to_bytes()
                    .len(),
            ))
        }
        .as_ref()
    );
    assert_eq!(
        luaur_ast::records::location::Location::new(
            luaur_ast::records::position::Position::new(3, 21),
            luaur_ast::records::position::Position::new(3, 27)
        ),
        method.name_location
    );
    assert!(unsafe {
        luaur_ast::rtti::ast_node_is::<luaur_ast::records::ast_type_function::AstTypeFunction>(
            method.ty as *mut luaur_ast::records::ast_node::AstNode,
        )
    });
    assert_eq!(
        luaur_ast::records::location::Location::new(
            luaur_ast::records::position::Position::new(3, 12),
            luaur_ast::records::position::Position::new(3, 54)
        ),
        method.location
    );
    assert!(method.is_method);

    let stat1 = unsafe { &*(*root.body.data.add(1)) };
    let subclass = unsafe {
        luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType,
        >(
            stat1 as *const luaur_ast::records::ast_stat::AstStat
                as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!subclass.is_null());
    assert!(unsafe { (*subclass).super_name.is_some() });
    assert_eq!(
        "Bar",
        unsafe {
            alloc::string::String::from_utf8_lossy(core::slice::from_raw_parts(
                (*subclass).name.value as *const u8,
                core::ffi::CStr::from_ptr((*subclass).name.value)
                    .to_bytes()
                    .len(),
            ))
        }
        .as_ref()
    );
    assert_eq!(
        "Foo",
        unsafe {
            alloc::string::String::from_utf8_lossy(core::slice::from_raw_parts(
                (*subclass).super_name.unwrap().value as *const u8,
                core::ffi::CStr::from_ptr((*subclass).super_name.unwrap().value)
                    .to_bytes()
                    .len(),
            ))
        }
        .as_ref()
    );

    assert_eq!(1, unsafe { (*subclass).props.size });
    let prop2 = unsafe { *(*subclass).props.data.add(0) };
    assert_eq!(
        "prop2",
        unsafe {
            alloc::string::String::from_utf8_lossy(core::slice::from_raw_parts(
                prop2.name.value as *const u8,
                core::ffi::CStr::from_ptr(prop2.name.value).to_bytes().len(),
            ))
        }
        .as_ref()
    );
    assert_eq!(
        luaur_ast::records::location::Location::new(
            luaur_ast::records::position::Position::new(7, 12),
            luaur_ast::records::position::Position::new(7, 17)
        ),
        prop2.name_location
    );
    assert!(unsafe {
        luaur_ast::rtti::ast_node_is::<luaur_ast::records::ast_type_reference::AstTypeReference>(
            prop2.ty as *mut luaur_ast::records::ast_node::AstNode,
        )
    });
    assert_eq!(
        luaur_ast::records::location::Location::new(
            luaur_ast::records::position::Position::new(7, 12),
            luaur_ast::records::position::Position::new(7, 25)
        ),
        prop2.location
    );
}
