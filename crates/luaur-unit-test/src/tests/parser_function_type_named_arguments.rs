#[cfg(test)]
#[test]
fn parser_function_type_named_arguments() {
    use crate::records::fixture::Fixture;

    {
        let mut fixture = Fixture::default();
        let result = fixture.parse_ex(
            &alloc::string::String::from(
                "type MyFunc = (a: number, b: string, c: number) -> string",
            ),
            &luaur_ast::records::parse_options::ParseOptions::default(),
        );
        let stat: *mut luaur_ast::records::ast_stat_block::AstStatBlock = result.root;
        assert!(!stat.is_null());
        let decl = unsafe {
            luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias>(
                *(&*stat).body.data as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        assert!(!decl.is_null());
        let func = unsafe {
            luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_function::AstTypeFunction>(
                (*decl).type_ptr as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        assert!(!func.is_null());
        assert_eq!(unsafe { (*func).arg_types.types.size }, 3);
        assert_eq!(unsafe { (*func).arg_names.size }, 3);
        assert!(!unsafe { (*func).arg_names.data.add(2).read().is_none() });
        let arg_name_c = unsafe { (*func).arg_names.data.add(2).read().unwrap() };
        assert_eq!(
            unsafe { core::ffi::CStr::from_ptr(arg_name_c.0.value).to_string_lossy() },
            "c"
        );
    }

    {
        let mut fixture = Fixture::default();
        let result = fixture.parse_ex(
            &alloc::string::String::from("type MyFunc = (a: number, string, c: number) -> string"),
            &luaur_ast::records::parse_options::ParseOptions::default(),
        );
        let stat: *mut luaur_ast::records::ast_stat_block::AstStatBlock = result.root;
        assert!(!stat.is_null());
        let decl = unsafe {
            luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias>(
                *(&*stat).body.data as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        assert!(!decl.is_null());
        let func = unsafe {
            luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_function::AstTypeFunction>(
                (*decl).type_ptr as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        assert!(!func.is_null());
        assert_eq!(unsafe { (*func).arg_types.types.size }, 3);
        assert_eq!(unsafe { (*func).arg_names.size }, 3);
        assert!(unsafe { (*func).arg_names.data.add(1).read().is_none() });
        assert!(!unsafe { (*func).arg_names.data.add(2).read().is_none() });
        let arg_name_c = unsafe { (*func).arg_names.data.add(2).read().unwrap() };
        assert_eq!(
            unsafe { core::ffi::CStr::from_ptr(arg_name_c.0.value).to_string_lossy() },
            "c"
        );
    }

    {
        let mut fixture = Fixture::default();
        let result = fixture.parse_ex(
            &alloc::string::String::from("type MyFunc = (a: number, string, number) -> string"),
            &luaur_ast::records::parse_options::ParseOptions::default(),
        );
        let stat: *mut luaur_ast::records::ast_stat_block::AstStatBlock = result.root;
        assert!(!stat.is_null());
        let decl = unsafe {
            luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias>(
                *(&*stat).body.data as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        assert!(!decl.is_null());
        let func = unsafe {
            luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_function::AstTypeFunction>(
                (*decl).type_ptr as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        assert!(!func.is_null());
        assert_eq!(unsafe { (*func).arg_types.types.size }, 3);
        assert_eq!(unsafe { (*func).arg_names.size }, 3);
        assert!(unsafe { (*func).arg_names.data.add(1).read().is_none() });
        assert!(unsafe { (*func).arg_names.data.add(2).read().is_none() });
    }

    {
        let mut fixture = Fixture::default();
        let result = fixture.parse_ex(
            &alloc::string::String::from("type MyFunc = (a: number, b: string, c: number) -> (d: number, e: string, f: number) -> string"),
            &luaur_ast::records::parse_options::ParseOptions::default(),
        );
        let stat: *mut luaur_ast::records::ast_stat_block::AstStatBlock = result.root;
        assert!(!stat.is_null());
        let decl = unsafe {
            luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias>(
                *(&*stat).body.data as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        assert!(!decl.is_null());
        let func = unsafe {
            luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_function::AstTypeFunction>(
                (*decl).type_ptr as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        assert!(!func.is_null());
        assert_eq!(unsafe { (*func).arg_types.types.size }, 3);
        assert_eq!(unsafe { (*func).arg_names.size }, 3);
        assert!(!unsafe { (*func).arg_names.data.add(2).read().is_none() });
        let arg_name_c = unsafe { (*func).arg_names.data.add(2).read().unwrap() };
        assert_eq!(
            unsafe { core::ffi::CStr::from_ptr(arg_name_c.0.value).to_string_lossy() },
            "c"
        );
        let explicit_pack = unsafe {
            luaur_ast::rtti::ast_node_as::<
                luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit,
            >((*func).return_types as *mut luaur_ast::records::ast_node::AstNode)
        };
        assert!(!explicit_pack.is_null());
        let func_ret = unsafe {
            luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_function::AstTypeFunction>(
                (*explicit_pack).type_list.types.data.add(0).read()
                    as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        assert!(!func_ret.is_null());
        assert_eq!(unsafe { (*func_ret).arg_types.types.size }, 3);
        assert_eq!(unsafe { (*func_ret).arg_names.size }, 3);
        assert!(!unsafe { (*func_ret).arg_names.data.add(2).read().is_none() });
        let arg_name_f = unsafe { (*func_ret).arg_names.data.add(2).read().unwrap() };
        assert_eq!(
            unsafe { core::ffi::CStr::from_ptr(arg_name_f.0.value).to_string_lossy() },
            "f"
        );
    }

    {
        let mut fixture = Fixture::default();
        fixture.match_parse_error(
            &alloc::string::String::from("type MyFunc = (a: number, b: string, c: number) -> (d: number, e: string, f: number)"),
            &alloc::string::String::from("Expected '->' when parsing function type, got <eof>"),
            None,
        );
    }

    {
        let mut fixture = Fixture::default();
        fixture.match_parse_error(
            &alloc::string::String::from(
                "type MyFunc = (number) -> (d: number) <a, b, c> -> number",
            ),
            &alloc::string::String::from("Expected '->' when parsing function type, got '<'"),
            None,
        );
    }
}
