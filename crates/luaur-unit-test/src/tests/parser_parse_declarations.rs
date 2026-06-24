#[cfg(test)]
#[test]
fn parser_parse_declarations() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let result = fixture.parse_ex(
        &alloc::string::String::from(
            "\n        declare foo: number\n        declare function bar(x: number): string\n        declare function var(...: any)\n    ",
        ),
        &ParseOptions::default(),
    );
    let root = unsafe { &*result.root };
    assert_eq!(root.body.size, 3);

    let global = unsafe {
        luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_stat_declare_global::AstStatDeclareGlobal,
        >(*root.body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode)
        .as_ref()
    };
    assert!(global.is_some());
    let global = global.unwrap();
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr(global.name.value).to_string_lossy() },
        "foo"
    );
    assert_eq!(
        global.name_location,
        luaur_ast::records::location::Location::new(
            luaur_ast::records::position::Position::new(1, 16),
            luaur_ast::records::position::Position::new(1, 19),
        )
    );
    assert!(!global.type_.is_null());

    let func = unsafe {
        luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_stat_declare_function::AstStatDeclareFunction,
        >(*root.body.data.add(1) as *mut luaur_ast::records::ast_node::AstNode)
        .as_ref()
    };
    assert!(func.is_some());
    let func = func.unwrap();
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr(func.name.value).to_string_lossy() },
        "bar"
    );
    assert_eq!(
        func.name_location,
        luaur_ast::records::location::Location::new(
            luaur_ast::records::position::Position::new(2, 25),
            luaur_ast::records::position::Position::new(2, 28),
        )
    );
    assert_eq!(func.params.types.size, 1);

    let ret_type_pack = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit>(func.ret_types as *mut luaur_ast::records::ast_node::AstNode).as_ref()
    };
    assert!(ret_type_pack.is_some());
    let ret_type_pack = ret_type_pack.unwrap();
    assert_eq!(ret_type_pack.type_list.types.size, 1);

    let var_func = unsafe {
        luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_stat_declare_function::AstStatDeclareFunction,
        >(*root.body.data.add(2) as *mut luaur_ast::records::ast_node::AstNode)
        .as_ref()
    };
    assert!(var_func.is_some());
    let var_func = var_func.unwrap();
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr(var_func.name.value).to_string_lossy() },
        "var"
    );
    assert_eq!(
        var_func.name_location,
        luaur_ast::records::location::Location::new(
            luaur_ast::records::position::Position::new(3, 25),
            luaur_ast::records::position::Position::new(3, 28),
        )
    );
    assert!(var_func.vararg);
    assert_eq!(
        var_func.vararg_location,
        luaur_ast::records::location::Location::new(
            luaur_ast::records::position::Position::new(3, 29),
            luaur_ast::records::position::Position::new(3, 32),
        )
    );

    fixture.match_parse_error(
        &alloc::string::String::from("declare function foo(x)"),
        &alloc::string::String::from("All declaration parameters must be annotated"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("declare foo"),
        &alloc::string::String::from(
            "Expected ':' when parsing global variable declaration, got <eof>",
        ),
        None,
    );
}
