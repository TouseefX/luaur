#[cfg(test)]
#[test]
fn parser_class_recovery_invalid_body_token() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::records::ast_class_method::AstClassMethod;
    use luaur_ast::records::ast_class_property::AstClassProperty;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat_class::AstStatClass;
    use luaur_ast::records::parse_options::ParseOptions;

    let _g = ScopedFastFlag::new(&luaur_common::FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    let src = alloc::string::String::from(
        "\nclass Foo\n    public x: number\n    blah\n    function bar() end\nend\n    ",
    );
    let result = fix.try_parse(&src, &ParseOptions::default());
    assert!(!result.errors.is_empty());

    assert_eq!(unsafe { (*result.root).body.size }, 1);
    let s0 = unsafe { *(*result.root).body.data.add(0) };
    let cls = unsafe { luaur_ast::rtti::ast_node_as::<AstStatClass>(s0 as *mut AstNode) };
    assert!(!cls.is_null());
    let cls = unsafe { &*cls };
    assert_eq!(cls.members.size, 2);

    let m1 = unsafe { &*cls.members.data.add(0) }.get_if::<AstClassProperty>();
    assert!(m1.is_some());
    assert!(m1.unwrap().name.operator_eq_c_char(c"x".as_ptr()));

    let m2 = unsafe { &*cls.members.data.add(1) }.get_if::<AstClassMethod>();
    assert!(m2.is_some());
    assert!(m2
        .unwrap()
        .function_name
        .operator_eq_c_char(c"bar".as_ptr()));
}
