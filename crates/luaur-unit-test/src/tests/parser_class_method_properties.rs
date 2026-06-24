#[cfg(test)]
#[test]
fn parser_class_method_properties() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let result1 = fixture.match_parse_error(
        &alloc::string::String::from(
            "\n        declare class Foo\n            -- method's first parameter must be 'self'\n            function method(foo: number)\n            function method2(self)\n        end\n        ",
        ),
        &alloc::string::String::from("'self' must be present as the unannotated first parameter"),
        None,
    );

    assert_eq!(1, unsafe { (*result1.root).body.size });

    let klass = unsafe {
        let node = (*result1.root).body.data.add(0);
        luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType,
        >(*node as *mut luaur_ast::records::ast_node::AstNode)
    };
    assert!(!klass.is_null());
    assert_eq!(2, unsafe { (*klass).props.size });

    let mut fixture2 = Fixture::default();
    let result2 = fixture2.match_parse_error(
        &alloc::string::String::from(
            "\n        declare class Foo\n            function method(self, foo)\n            function method2()\n        end\n        ",
        ),
        &alloc::string::String::from("All declaration parameters aside from 'self' must be annotated"),
        None,
    );

    assert_eq!(1, unsafe { (*result2.root).body.size });

    let klass2 = unsafe {
        let node = (*result2.root).body.data.add(0);
        luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType,
        >(*node as *mut luaur_ast::records::ast_node::AstNode)
    };
    assert!(!klass2.is_null());
    assert_eq!(2, unsafe { (*klass2).props.size });
}
