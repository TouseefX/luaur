#[cfg(test)]
#[test]
fn parser_export_value_rfc() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag::LuauConst2;
    use luaur_common::FFlag::LuauExportValueSyntax;

    let mut fixture = Fixture::fixture_bool(false);
    let _sff_luau_export_value_syntax = ScopedFastFlag::new(&LuauExportValueSyntax, true);
    let _sff_luau_const2 = ScopedFastFlag::new(&LuauConst2, true);

    let source = alloc::string::String::from(
        "export local version = \"1.0.0\"\n\
         export const TAU = math.pi * 2\n\
         export local settings: Settings = getSettings()\n\
         export local a, b, c = 1, 2, 3\n\
         export local d\n\
         \n\
         export function add(a: number, b: number): number\n\
             return a + b\n\
         end\n\
         \n\
         export local f, g\n\
         function f()\n\
             return g()\n\
         end\n\
         \n\
         function g()\n\
             return 42\n\
         end\n\
         \n\
         local function ret(): (string, number, boolean)\n\
             return \"heh\", 42, false\n\
         end\n\
         export local x, y, z = ret()\n",
    );

    let result = fixture.parse(
        &source,
        &luaur_ast::records::parse_options::ParseOptions::parse_options(),
    );

    assert!(!result.is_null());
    assert_eq!(unsafe { (*result).body.size }, 11);

    let version = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_local::AstStatLocal>(
            *(*result).body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!version.is_null());
    assert!(unsafe { (*version).is_exported });
    assert!(!unsafe { (*version).is_const });
    assert_eq!(unsafe { (*version).vars.size }, 1);
    assert!(unsafe { (**(*version).vars.data.add(0)).is_exported });
    assert!(!unsafe { (**(*version).vars.data.add(0)).is_const });

    let tau = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_local::AstStatLocal>(
            *(*result).body.data.add(1) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!tau.is_null());
    assert!(unsafe { (*tau).is_exported });
    assert!(unsafe { (*tau).is_const });
    assert_eq!(unsafe { (*tau).vars.size }, 1);
    assert!(unsafe { (**(*tau).vars.data.add(0)).is_exported });
    assert!(unsafe { (**(*tau).vars.data.add(0)).is_const });

    let settings = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_local::AstStatLocal>(
            *(*result).body.data.add(2) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!settings.is_null());
    assert!(unsafe { (*settings).is_exported });
    assert!(!unsafe { (*settings).is_const });
    assert_eq!(unsafe { (*settings).vars.size }, 1);
    assert!(!unsafe { (**(*settings).vars.data.add(0)).annotation.is_null() });

    let abc = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_local::AstStatLocal>(
            *(*result).body.data.add(3) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!abc.is_null());
    assert!(unsafe { (*abc).is_exported });
    assert!(!unsafe { (*abc).is_const });
    assert_eq!(unsafe { (*abc).vars.size }, 3);
    for i in 0..unsafe { (*abc).vars.size } as usize {
        assert!(unsafe { (**(*abc).vars.data.add(i)).is_exported });
        assert!(!unsafe { (**(*abc).vars.data.add(i)).is_const });
    }

    let d = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_local::AstStatLocal>(
            *(*result).body.data.add(4) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!d.is_null());
    assert!(unsafe { (*d).is_exported });
    assert!(!unsafe { (*d).is_const });
    assert_eq!(unsafe { (*d).vars.size }, 1);
    assert_eq!(unsafe { (*d).values.size }, 0);
    assert!(unsafe { (**(*d).vars.data.add(0)).is_exported });

    let add = unsafe {
        luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_stat_local_function::AstStatLocalFunction,
        >(*(*result).body.data.add(5) as *mut luaur_ast::records::ast_node::AstNode)
    };
    assert!(!add.is_null());
    assert!(unsafe { (*(*add).name).is_exported });
    assert!(unsafe { (*(*add).name).is_const });

    let forward_decls = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_local::AstStatLocal>(
            *(*result).body.data.add(6) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!forward_decls.is_null());
    assert!(unsafe { (*forward_decls).is_exported });
    assert!(!unsafe { (*forward_decls).is_const });
    assert_eq!(unsafe { (*forward_decls).vars.size }, 2);
    assert_eq!(unsafe { (*forward_decls).values.size }, 0);
    for i in 0..unsafe { (*forward_decls).vars.size } as usize {
        assert!(unsafe { (**(*forward_decls).vars.data.add(i)).is_exported });
        assert!(!unsafe { (**(*forward_decls).vars.data.add(i)).is_const });
    }

    assert!(unsafe {
        luaur_ast::rtti::ast_node_is::<luaur_ast::records::ast_stat_function::AstStatFunction>(
            *(*result).body.data.add(7) as *mut luaur_ast::records::ast_node::AstNode,
        )
    });
    assert!(unsafe {
        luaur_ast::rtti::ast_node_is::<luaur_ast::records::ast_stat_function::AstStatFunction>(
            *(*result).body.data.add(8) as *mut luaur_ast::records::ast_node::AstNode,
        )
    });

    // C++ reads `xyz` from body.data[3] (re-checking the `export local a, b, c`
    // statement — the name is misleading but faithful to upstream).
    let xyz = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_local::AstStatLocal>(
            *(*result).body.data.add(3) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!xyz.is_null());
    assert!(unsafe { (*xyz).is_exported });
    assert!(!unsafe { (*xyz).is_const });
    assert_eq!(unsafe { (*xyz).vars.size }, 3);
    for i in 0..unsafe { (*xyz).vars.size } as usize {
        assert!(unsafe { (**(*xyz).vars.data.add(i)).is_exported });
        assert!(!unsafe { (**(*xyz).vars.data.add(i)).is_const });
    }

    let source2 = alloc::string::String::from(
        "export type Config = {\n\
         debug: boolean,\n\
         timeout: number,\n\
         }\n\
         \n\
         return {\n\
         debug = false,\n\
         timeout = 5,\n\
         }\n",
    );

    let _result2 = fixture.parse(
        &source2,
        &luaur_ast::records::parse_options::ParseOptions::parse_options(),
    );
}
